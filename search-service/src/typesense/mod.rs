//! Typesense HTTP API client.
//!
//! Handles collection bootstrap, document upsert, and search queries.
//! All communication uses Typesense's REST API with JSON payloads.

use anyhow::{bail, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Configuration for connecting to a Typesense node.
#[derive(Clone, Debug)]
pub struct TypesenseConfig {
    pub url: String,
    pub api_key: String,
    pub collection: String,
}

impl TypesenseConfig {
    pub fn from_env() -> Self {
        Self {
            url: std::env::var("TYPESENSE_URL")
                .unwrap_or_else(|_| "http://localhost:8108".into()),
            api_key: std::env::var("TYPESENSE_API_KEY")
                .expect("TYPESENSE_API_KEY must be set"),
            collection: std::env::var("TYPESENSE_COLLECTION")
                .unwrap_or_else(|_| "docmind_chunks".into()),
        }
    }
}

/// A single document chunk stored in Typesense.
#[derive(Debug, Serialize, Deserialize)]
pub struct ChunkDocument {
    pub id: String,
    pub document_id: String,
    pub chunk_index: i64,
    pub content: String,
}

/// One search hit returned by Typesense.
#[derive(Debug, Deserialize)]
pub struct SearchHit {
    pub document: ChunkDocument,
    #[serde(default)]
    pub text_match: u64,
}

pub struct TypesenseClient {
    http: Client,
    cfg: TypesenseConfig,
}

impl TypesenseClient {
    pub fn new(cfg: TypesenseConfig) -> Self {
        Self {
            http: Client::new(),
            cfg,
        }
    }

    fn collection_url(&self) -> String {
        format!("{}/collections/{}", self.cfg.url, self.cfg.collection)
    }

    fn documents_url(&self) -> String {
        format!("{}/documents", self.collection_url())
    }

    /// Create the collection if it does not already exist.
    pub async fn ensure_collection(&self) -> Result<()> {
        let schema = json!({
            "name": self.cfg.collection,
            "fields": [
                { "name": "id",           "type": "string" },
                { "name": "document_id",  "type": "string", "facet": true },
                { "name": "chunk_index",  "type": "int64" },
                { "name": "content",      "type": "string" }
            ],
            "default_sorting_field": "chunk_index"
        });

        let res = self
            .http
            .post(format!("{}/collections", self.cfg.url))
            .header("X-TYPESENSE-API-KEY", &self.cfg.api_key)
            .json(&schema)
            .send()
            .await?;

        // 409 = already exists — that's fine
        if !res.status().is_success() && res.status().as_u16() != 409 {
            let body = res.text().await.unwrap_or_default();
            bail!("failed to create Typesense collection: {body}");
        }

        Ok(())
    }

    /// Import a batch of chunks for a document.
    pub async fn index_chunks(
        &self,
        document_id: &str,
        chunks: &[String],
    ) -> Result<usize> {
        let docs: Vec<Value> = chunks
            .iter()
            .enumerate()
            .map(|(i, content)| {
                json!({
                    "id":          format!("{document_id}_{i}"),
                    "document_id": document_id,
                    "chunk_index": i as i64,
                    "content":     content
                })
            })
            .collect();

        // JSONL batch import
        let jsonl = docs
            .iter()
            .map(|d| d.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        let res = self
            .http
            .post(format!("{}/import?action=upsert", self.documents_url()))
            .header("X-TYPESENSE-API-KEY", &self.cfg.api_key)
            .header("Content-Type", "text/plain")
            .body(jsonl)
            .send()
            .await?;

        if !res.status().is_success() {
            let body = res.text().await.unwrap_or_default();
            bail!("Typesense import error: {body}");
        }

        Ok(docs.len())
    }

    /// Delete all chunks belonging to a document.
    pub async fn delete_document(&self, document_id: &str) -> Result<()> {
        let res = self
            .http
            .delete(format!(
                "{}/documents?filter_by=document_id:={}",
                self.documents_url(),
                document_id
            ))
            .header("X-TYPESENSE-API-KEY", &self.cfg.api_key)
            .send()
            .await?;

        if !res.status().is_success() {
            let body = res.text().await.unwrap_or_default();
            bail!("Typesense delete error: {body}");
        }

        Ok(())
    }

    /// Full-text search across indexed chunks.
    pub async fn search(
        &self,
        query: &str,
        page: usize,
        per_page: usize,
    ) -> Result<Vec<SearchHit>> {
        #[derive(Deserialize)]
        struct SearchResponse {
            hits: Vec<SearchHit>,
        }

        let res = self
            .http
            .get(format!("{}/documents/search", self.collection_url()))
            .header("X-TYPESENSE-API-KEY", &self.cfg.api_key)
            .query(&[
                ("q", query),
                ("query_by", "content"),
                ("page", &page.to_string()),
                ("per_page", &per_page.to_string()),
                ("highlight_full_fields", "content"),
            ])
            .send()
            .await?;

        if !res.status().is_success() {
            let body = res.text().await.unwrap_or_default();
            bail!("Typesense search error: {body}");
        }

        let parsed: SearchResponse = res.json().await?;
        Ok(parsed.hits)
    }
}

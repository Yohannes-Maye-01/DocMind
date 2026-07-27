use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

use crate::index::InMemoryIndex;
use crate::parser;
use crate::search;

/// Build the axum router with all search-service endpoints.
pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/parse", post(parse))
        .route("/index", post(index_document))
        .route("/search", post(search_documents))
}

// ── Health ────────────────────────────────────────────────────────────────────

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "status": "ok" })))
}

// ── Parse ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct ParseRequest {
    filename: String,
    content: String, // base64-encoded file bytes
}

#[derive(Serialize)]
struct ParseResponse {
    chunks: Vec<String>,
    chunk_count: usize,
}

async fn parse(Json(req): Json<ParseRequest>) -> impl IntoResponse {
    match parser::parse(&req.filename, &req.content) {
        Ok(chunks) => {
            let count = chunks.len();
            (StatusCode::OK, Json(serde_json::to_value(ParseResponse { chunks, chunk_count: count }).unwrap())).into_response()
        }
        Err(e) => (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(serde_json::json!({ "error": e.to_string() })),
        ).into_response(),
    }
}

// ── Index ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct IndexRequest {
    document_id: String,
    chunks: Vec<String>,
}

#[derive(Serialize)]
struct IndexResponse {
    indexed: usize,
}

async fn index_document(Json(req): Json<IndexRequest>) -> impl IntoResponse {
    let mut idx = InMemoryIndex::global();
    let count = idx.insert(&req.document_id, &req.chunks);
    (StatusCode::OK, Json(serde_json::json!({ "indexed": count })))
}

// ── Search ────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct SearchRequest {
    query: String,
    page: Option<usize>,
    limit: Option<usize>,
}

async fn search_documents(Json(req): Json<SearchRequest>) -> impl IntoResponse {
    let idx = InMemoryIndex::global();
    let page = req.page.unwrap_or(1).max(1);
    let limit = req.limit.unwrap_or(20).clamp(1, 100);

    let results = search::query(&idx, &req.query, page, limit);
    (StatusCode::OK, Json(serde_json::json!({
        "results": results.items,
        "total":   results.total,
        "page":    results.page,
    })))
}

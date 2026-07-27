use std::collections::HashMap;

use crate::index::{tokenize, InMemoryIndex};

/// A single search result item.
#[derive(serde::Serialize)]
pub struct SearchResult {
    pub document_id: String,
    pub chunk_index: usize,
    pub snippet: String,
    pub score: f64,
}

/// Paginated search results.
pub struct SearchPage {
    pub items: Vec<SearchResult>,
    pub total: usize,
    pub page: usize,
}

/// Execute a keyword search against the in-memory index using TF-IDF scoring.
pub fn query(idx: &InMemoryIndex, query: &str, page: usize, limit: usize) -> SearchPage {
    let query_tokens: Vec<String> = tokenize(query).collect();
    if query_tokens.is_empty() {
        return SearchPage { items: vec![], total: 0, page };
    }

    // Accumulate raw scores per (doc_id, chunk_index)
    let mut scores: HashMap<(String, usize), f64> = HashMap::new();

    for token in &query_tokens {
        let postings = idx.lookup(token);
        if postings.is_empty() {
            continue;
        }
        // Simple IDF: log(1 + 1 / df)
        let idf = (1.0 + 1.0_f64 / postings.len() as f64).ln();

        for (doc_id, chunk_idx) in postings {
            *scores.entry((doc_id.clone(), *chunk_idx)).or_default() += idf;
        }
    }

    // Sort by score descending
    let mut ranked: Vec<_> = scores.into_iter().collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let total = ranked.len();
    let start = (page - 1) * limit;
    let items = ranked
        .into_iter()
        .skip(start)
        .take(limit)
        .filter_map(|((doc_id, chunk_idx), score)| {
            let chunk = idx.get_chunk(&doc_id, chunk_idx)?;
            Some(SearchResult {
                document_id: doc_id,
                chunk_index: chunk_idx,
                snippet: truncate(&chunk.content, 300),
                score,
            })
        })
        .collect();

    SearchPage { items, total, page }
}

fn truncate(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_chars).collect();
        format!("{truncated}…")
    }
}

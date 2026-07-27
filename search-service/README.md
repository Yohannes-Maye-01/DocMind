# DocMind – Search Service (Rust)

High-performance text processing and keyword search engine. Handles document parsing, chunking, and in-memory BTreeMap indexing.

## Stack
- **Language:** Rust 1.79 (edition 2021)
- **HTTP server:** axum 0.7 + tokio
- **PDF parsing:** lopdf
- **Markdown parsing:** pulldown-cmark

## Structure
```
search-service/
├── src/
│   ├── api/mod.rs      # HTTP routes: /health /parse /index /search
│   ├── index/mod.rs    # In-memory BTreeMap inverted index
│   ├── parser/mod.rs   # .txt / .md / .pdf → chunked text
│   ├── search/mod.rs   # TF-IDF ranking + pagination
│   └── main.rs         # axum server entry point
├── Cargo.toml
└── Dockerfile
```

## Running locally
```bash
PORT=8081 cargo run
```

## Endpoints
| Method | Path      | Description |
|--------|-----------|-------------|
| GET    | /health   | Service health |
| POST   | /parse    | Parse file → text chunks |
| POST   | /index    | Index chunks into memory |
| POST   | /search   | Keyword search with pagination |

## Code Style
- `cargo clippy -- -D warnings` before every commit
- `cargo fmt`
- 80% unit test coverage target (Phase 6)

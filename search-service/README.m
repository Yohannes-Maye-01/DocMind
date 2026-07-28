# DocMind Search Service

A high-performance search and retrieval service built in Rust. This service handles lightning-fast text and vector database operations using TypeSense for the DocMind RAG (Retrieval-Augmented Generation) pipeline.

## Features

- **Blazing Fast Retrieval**: Powered by Rust for ultra-low latency.
- **Hybrid Search**: Combines keyword search with dense vector embeddings.
- **TypeSense Integration**: Native client integration for collection management and indexing.
- **RAG Ready**: Provides optimized endpoints for feeding context into the `ai-service`.

## Prerequisites

- [Rust](https://rust-lang.org) (latest stable version)
- [TypeSense Server](https://typesense.org) (running locally or in Docker)

## Configuration

Create a `.env` file in the root of this directory:

```env
TYPESENSE_API_KEY=xyz123
TYPESENSE_HOST=127.0.0.1
TYPESENSE_PORT=8108
TYPESENSE_PROTOCOL=http
SERVER_PORT=8080
```

## Getting Started

### 1. Run TypeSense Locally
```bash
docker run -p 8108:8108 -v/tmp/data:/data typesense/typesense:26.0 \
  --data-dir /data --api-key=xyz123 --enable-cors
```

### 2. Build and Run the Service
```bash
# Development mode
cargo run

# Production release
cargo build --release
./target/release/search-service
```

### 3. Run Tests
```bash
cargo test
```

## API Endpoints

| Method | Endpoint | Description |
| :--- | :--- | :--- |
| `POST` | `/api/v1/index` | Index a new document chunk |
| `POST` | `/api/v1/search` | Query documents (hybrid vector + text) |
| `DELETE` | `/api/v1/documents/:id` | Delete a document from the index |
| `GET` | `/health` | Service health check |


# Architecture Design Document
DocMind AI

## 1. Architectural Overview

DocMind follows a microservices-inspired polyglot architecture optimized for modularity, performance, and local execution on an 8GB RAM machine. Each service is chosen for its specific strengths:

- **TypeScript/React**: Responsive client-side interface for document management and Q&A interaction.
- **Go**: Fast, lightweight API gateway and orchestrator for all services.
- **Rust**: High-performance text processing, parsing, and full-text indexing.
- **Python**: Flexible AI/ML layer for embeddings and LLM inference via Ollama.
- **PostgreSQL**: Reliable relational storage for users, metadata, and document records.
- **TypeSense**: Open-source vector search engine for semantic search and embedding similarity.
- **Ollama**: Lightweight, local LLM inference engine (runs separately, managed via Docker Compose).

---

## 2. System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                      Client Layer                           │
│                  TypeScript / React (UI)                    │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           │ HTTP / REST (JSON)
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                     API Gateway Layer                       │
│                        Go Backend                           │
│       - Authentication (JWT)  - Request Orchestration       │
└──┬─────────────────────────────────────────────────────┬───┘
   │                                                     │
   │ Document Ingestion                   Search / Q&A Requests
   ▼                                                     ▼
┌────────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│Search Engine(Rust) │    │TypeSense Vector  │    │ AI Service(Py)  │
│- Text Parsing      │◄──►│  Search          │◄──►│- Embedding Gen. │
│- Tokenization      │    │- Vector Storage  │    │- RAG Pipeline   │
│- Full-Text Index   │    │- Semantic Search │    │- LLM Inference  │
└────────┬───────────┘    └────────┬─────────┘    └────────┬────────┘
         │                         │                       │
         │                         │  (via Ollama)         │
         │                         │                       │
         │              ┌──────────▼──────────┐            │
         │              │ Ollama LLM Engine   │            │
         │              │ (Local, Offline)    │            │
         │              └─────────────────────┘            │
         │                                                 │
         └─────────────────────┬──────────────────────────┘
                               │
                               ▼
         ┌─────────────────────────────────────┐
         │      Persistence Layer              │
         ├─────────────────────────────────────┤
         │ PostgreSQL                          │
         │ - User Accounts                     │
         │ - File Metadata & Status            │
         │ - Document Chunks                   │
         │ - Search History                    │
         └─────────────────────────────────────┘
```

---

## 3. Core Component Responsibilities

### A. Frontend (TypeScript / React)
- Renders the dashboard, document upload interface, search bar, and chat interface.
- Communicates exclusively with the Go Backend API via RESTful endpoints.
- Displays search results with semantic relevance highlighting.
- Shows chat/Q&A responses with source document references.

### B. Backend API (Go)
- Acts as the single entry point (API Gateway) for the frontend.
- Manages user authentication, session validation, and database interactions with PostgreSQL.
- Orchestrates data flow:
  - Dispatches uploaded files to the Rust service for parsing and chunking.
  - Sends chunks to TypeSense for vector indexing.
  - Calls Python service for embedding generation and semantic search.
  - Handles full-text search queries against both Rust and TypeSense indexes.

### C. Search Engine Service (Rust)
- Handles heavy text manipulation, tokenization, and document chunking (512-token window with 50-token overlap).
- Maintains in-memory full-text index using BTreeMap or similar data structures.
- Provides keyword/TF-IDF search for primary document retrieval.
- Chosen for Rust's memory safety and zero-cost abstractions, ensuring minimal RAM consumption.
- **New**: Prepares chunk embeddings for TypeSense ingestion.

### D. TypeSense (Vector Search Engine)
- **Purpose**: Lightning-fast semantic search using vector embeddings.
- **Key Features**:
  - Stores and indexes document chunk embeddings as vectors.
  - Provides similarity search (cosine/L2 distance).
  - Scales efficiently on 8GB RAM constraint.
  - Supports filtering and metadata-based queries (document ID, chunk position).
  - HTTP/REST API for easy integration with Go backend.
- **Integration Flow**:
  1. Python service generates embeddings for document chunks.
  2. Go backend sends embeddings to TypeSense for indexing.
  3. On search queries, Python generates query embedding.
  4. TypeSense performs fast similarity search, returns top-k results.
  5. Results enriched with full-text context from Rust or PostgreSQL.

### E. AI Service (Python)
- Manages text vectorization (generating embeddings for chunks) using Sentence Transformers (`all-MiniLM-L6-v2` or similar).
- Interfaces with Ollama for lightweight local LLM inference.
- Handles Retrieval-Augmented Generation (RAG) by:
  1. Accepting user questions.
  2. Generating query embeddings.
  3. Sending to TypeSense for semantic chunk retrieval.
  4. Combining retrieved chunks into a context prompt.
  5. Querying Ollama LLM with the prompt.
  6. Returning the answer with source references.
- Operates entirely offline within the 8GB RAM limit.

### F. Ollama (Local LLM Runtime)
- Lightweight, locally hosted inference engine for large language models.
- Recommended models:
  - **Mistral 7B Q4** (~5GB) – Good balance of quality and speed.
  - **Phi 2 Q4** (~3GB) – Faster alternative.
- Runs in a separate Docker container with memory limits.
- Called by Python service via HTTP API during RAG queries.

### G. Database (PostgreSQL)
- Stores relational entities:
  - User credentials and sessions.
  - Project/document metadata (filename, upload date, size, status).
  - Document chunks with text and metadata (position, token count).
  - Search history and user interactions (optional, for future analytics).
- Indexed for fast lookups by document ID, chunk ID, and user.

---

## 4. Data Flow Scenarios

### Scenario 1: Document Upload & Ingestion

```
1. User uploads file (.txt, .md, or .pdf) via React frontend.
   ↓
2. Go Backend receives file, saves metadata to PostgreSQL.
   ↓
3. Go Backend sends raw file to Rust Service.
   ↓
4. Rust Service:
   - Parses text (handles .txt, .md, .pdf formats).
   - Breaks into chunks (512-token window, 50-token overlap).
   - Creates full-text index (BTreeMap).
   - Returns chunk list to Go Backend.
   ↓
5. Go Backend sends chunks to Python Service.
   ↓
6. Python Service:
   - Generates embeddings for each chunk.
   - Returns embedding vectors to Go Backend.
   ↓
7. Go Backend sends vectors to TypeSense:
   - Index collections: one per document or shared with metadata.
   - Stores: chunk text, embedding vector, source doc ID, position.
   ↓
8. Go Backend updates PostgreSQL document status → "indexed".
   ↓
9. Frontend notified → document ready for search/Q&A.
```

### Scenario 2: Keyword/Full-Text Search

```
1. User enters search query in frontend.
   ↓
2. Go Backend receives query.
   ↓
3. Go Backend queries Rust full-text index.
   ↓
4. Rust returns ranked results (TF-IDF or BM25).
   ↓
5. Go Backend enriches results with document metadata from PostgreSQL.
   ↓
6. Frontend displays ranked list with snippets and source document.
```

### Scenario 3: Semantic Search (Vector-Based)

```
1. User enters search query in frontend.
   ↓
2. Go Backend receives query.
   ↓
3. Go Backend sends query to Python Service.
   ↓
4. Python Service generates embedding for the query.
   ↓
5. Python sends query embedding to TypeSense.
   ↓
6. TypeSense performs similarity search (top-k results).
   ↓
7. Go Backend enriches results with full-text context from PostgreSQL.
   ↓
8. Frontend displays semantically relevant results.
```

### Scenario 4: RAG-Powered Q&A

```
1. User submits natural language question via chat interface.
   ↓
2. Go Backend receives question.
   ↓
3. Go Backend sends question to Python Service.
   ↓
4. Python Service:
   - Generates embedding for the question.
   - Sends to TypeSense for semantic chunk retrieval (top 5-10 chunks).
   ↓
5. TypeSense returns relevant chunks with metadata.
   ↓
6. Python Service:
   - Retrieves full chunk text from PostgreSQL.
   - Structures RAG prompt: [context chunks] + [question].
   - Sends prompt to Ollama LLM.
   ↓
7. Ollama processes prompt, generates response.
   ↓
8. Python Service returns answer with source references to Go Backend.
   ↓
9. Frontend displays answer with links to source documents/chunks.
```

---

## 5. Deployment & Local Infrastructure

### Docker Compose Services

```yaml
services:
  postgres:        # PostgreSQL 14+
  go-backend:      # Go API Gateway
  rust-service:    # Rust search engine
  python-service:  # Python AI/embeddings
  typesense:       # TypeSense vector search
  ollama:          # Local LLM inference
  frontend:        # React UI
```

### Resource Allocation (8GB Total)
- **PostgreSQL**: 500MB
- **Go Backend**: 200MB
- **Rust Service**: 300MB
- **Python Service**: 1.5GB
- **TypeSense**: 500MB
- **Ollama (LLM)**: 3.5GB
- **Frontend/System**: 1GB
- **Headroom**: ~500MB

### Environment Configuration
- Managed via secure `.env` files injected at runtime.
- Service endpoints, API keys, model names, memory limits.
- Docker Compose automatically handles service discovery and networking.

---

## 6. Technology Rationale

| Component | Technology | Why? |
|-----------|-----------|------|
| Search (Full-Text) | Rust | Memory-safe, ultra-fast text processing, minimal overhead |
| Search (Semantic) | TypeSense | Open-source, lightweight, efficient vector search in-memory |
| API Gateway | Go | Concurrent request handling, fast HTTP, low memory footprint |
| Frontend | React/TypeScript | Modern, responsive UI, excellent ecosystem |
| Embeddings & RAG | Python | Rich ML libraries, Sentence Transformers, easy Ollama integration |
| Local LLM | Ollama | Lightweight, offline-capable, supports popular open models |
| Database | PostgreSQL | Reliable, ACID guarantees, excellent JSON support |
| Orchestration | Docker Compose | Simple, local-first, no external dependencies |

---

## 7. Key Design Decisions

1. **Polyglot Architecture**: Each language chosen for its strength, traded off complexity for performance and correctness.

2. **Dual Search Strategy**: 
   - Full-text (Rust/keyword) for exact matches and recall.
   - Vector search (TypeSense) for semantic understanding and relevance.
   - Can be combined (hybrid search) for best results.

3. **Local-First**: All components run locally; no external APIs. Entire system works offline.

4. **Chunk-Based Processing**: Documents broken into 512-token chunks with 50-token overlap to balance context and performance.

5. **Embedding Caching**: Embeddings stored in TypeSense and PostgreSQL for fast retrieval without regeneration.

6. **Memory-Conscious**: TypeSense chosen over Qdrant/Milvus for lighter footprint on 8GB machine.

---

## 8. Future Enhancements

- Distributed vector search (if scaling beyond local machine).
- Multi-modal embeddings (image + text support).
- Real-time collaboration features.
- Fine-tuned local models for domain-specific Q&A.
- API for external integrations.


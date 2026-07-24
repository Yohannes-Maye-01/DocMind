Software Requirements Specification (SRS)
DocMind AI

1. Introduction
Project Name: DocMind

Objective: Build an AI-powered document intelligence platform enabling users to upload files, execute high-speed searches, and interact with document content through natural language Q&A.

Target Environment: Local development on an 8GB RAM PC, operating entirely offline with free, open-source tools.

2. User Persona & Scope
Primary User: A single developer/user managing local documentation, technical notes, or research papers who needs instant search and contextual AI summaries without relying on paid cloud APIs.

3. Functional Requirements

FR-01: Document Management (CRUD)
FR-01.1: The system must allow users to upload documents via the frontend interface with file size limits (max 50MB per document).
FR-01.2: Supported initial file formats must include Plain Text (.txt), Markdown (.md), and Portable Document Format (.pdf).
FR-01.3: The backend must parse uploaded documents into manageable text chunks (512-token sliding window with 50-token overlap) for indexing and vector embedding.
FR-01.4: Users must be able to view a list of all uploaded documents and delete documents when no longer needed.
FR-01.5: Documents must be stored in PostgreSQL with metadata (upload date, file size, chunk count, status).

FR-02: Search Engine Service (Rust)
FR-02.1: The Rust search service must index parsed document chunks for rapid retrieval using an in-memory index (BTreeMap or similar).
FR-02.2: The system must support keyword/full-text search and prepare structured data payloads for vector similarity search.
FR-02.3: Search queries initiated from the frontend through the Go backend must return relevant document snippets within 500ms local latency (SLA).
FR-02.4: Search results must be ranked by relevance score and paginated (20 results per page).

FR-03: AI Service & RAG Q&A (Python)
FR-03.1: The Python AI service must generate text embeddings using a lightweight model (Sentence Transformers 'all-MiniLM-L6-v2' or equivalent; ~85MB).
FR-03.2: The system must implement a Retrieval-Augmented Generation (RAG) pipeline: fetch top 5 relevant chunks via the search engine and inject them into a quantized LLM prompt.
FR-03.3: The AI service must use a quantized local LLM (Ollama with Mistral 7B or Phi 2 quantized to 4-bit; ~5GB max).
FR-03.4: The AI service must provide natural language answers to user queries accompanied by source document references (document name, page/chunk number).
FR-03.5: Query responses must complete within 3 seconds (SLA).

FR-04: User Authentication & Security
FR-04.1: For single-user local deployment, authentication is optional but required infrastructure for future multi-user scaling.
FR-04.2: If implemented, session management must use JWT tokens with 24-hour expiration and refresh token rotation.
FR-04.3: Environment variables and secrets (database passwords, API configs) must be kept secure using .env files and excluded from version control.
FR-04.4: All inter-service communication must use internal network only (Docker network isolation).

FR-05: Data Persistence & Export
FR-05.1: Users must be able to export indexed documents and embeddings to a backup file (.zip format).
FR-05.2: System must support importing previously backed-up data to restore state.

4. Non-Functional Requirements & Constraints

NFR-01: Performance & Resource Efficiency (8GB RAM Constraint)
NFR-01.1: Memory budget allocation:
  - PostgreSQL: 500MB
  - Go API: 200MB
  - Rust Service: 300MB
  - Python Service (embeddings): 1.5GB
  - Python Service (LLM inference): 3.5GB (via Ollama container)
  - Frontend/System: ~1GB
  - Headroom: ~500MB

NFR-01.2: AI models must be lightweight or quantized:
  - Embedding Model: Sentence Transformers all-MiniLM-L6-v2 (~85MB)
  - LLM: Ollama with Mistral 7B Q4 (~5GB) or Phi 2 Q4 (~3GB)
  - Models run via Ollama service in separate container with resource limits.

NFR-01.3: Disk space requirement: minimum 15GB (10GB for LLM model + embeddings, 5GB for document storage/database).

NFR-02: Modularity & Architecture
NFR-02.1: The system must maintain strict separation of concerns:
  - TypeScript/React: Frontend presentation and user interactions (port 3000)
  - Go API: Core routing, authentication, and service orchestration (port 8080)
  - Rust Service: High-performance indexing and text processing (port 8081)
  - Python Service: ML, embedding, and LLM inference (port 8082)
  - Ollama Service: Local LLM inference backend (port 11434)
  - PostgreSQL: Relational data and metadata persistence (port 5432)

NFR-02.2: Inter-service Communication Protocol:
  - Go ↔ Rust: HTTP REST API (JSON payloads)
  - Go ↔ Python: HTTP REST API (JSON payloads)
  - Python ↔ Ollama: HTTP REST API (Ollama native format)
  - All services communicate via Docker internal network (no external exposure)

NFR-02.3: Error Handling & Resilience:
  - All services must implement exponential backoff retry logic (3 retries, 1s-5s delays)
  - Graceful degradation: if one service is unavailable, system returns meaningful error to user
  - Health check endpoints on all services (/health) for monitoring

NFR-03: Performance Benchmarks & SLAs
NFR-03.1: Search latency: < 500ms for queries up to 100,000 chunks
NFR-03.2: RAG Q&A response: < 3 seconds end-to-end (search + embedding + LLM inference)
NFR-03.3: Document upload + chunking: < 10 seconds for 10MB PDF
NFR-03.4: System must handle concurrent operations from single user without degradation

NFR-04: Security
NFR-04.1: Password hashing: bcrypt with cost factor 12 (if auth is implemented)
NFR-04.2: JWT token structure: HS256 algorithm, 24-hour expiration, refresh tokens rotated every 3 days
NFR-04.3: CORS: Disabled (local frontend only) or restricted to localhost:3000
NFR-04.4: No sensitive data (API keys, passwords) logged or stored in plain text
NFR-04.5: Database queries must use parameterized statements to prevent SQL injection

NFR-05: Maintainability & Documentation
NFR-05.1: Codebases must follow professional standards:
  - Go: idiomatic Go (gofmt, golint, 80-char line limit where practical)
  - Rust: clippy lints, rustfmt, memory safety without unsafe blocks where possible
  - Python: PEP 8 style guide, type hints, mypy type checking
  - TypeScript: ESLint + Prettier, strict mode enabled, no any types

NFR-05.2: Architecture documentation must include:
  - Data flow diagrams (mermaid or ASCII)
  - Service interaction diagrams
  - Memory profiling baseline (per service)
  - Local development setup guide (Docker Compose, .env template)
  - API endpoint reference (OpenAPI/Swagger)

NFR-06: Testing & Quality Assurance
NFR-06.1: Minimum test coverage:
  - Go API: 70% unit test coverage
  - Rust Service: 80% unit test coverage
  - Python Service: 75% unit test coverage

NFR-06.2: Integration testing: Full RAG pipeline end-to-end test with sample documents
NFR-06.3: Load testing: Verify system stability with 100 concurrent single-user queries

NFR-07: Data Management
NFR-07.1: Document data retention: Local storage only; no cloud sync or telemetry
NFR-07.2: Backup strategy: Manual export via UI; automatic incremental backups to local .zip every 7 days (optional)
NFR-07.3: Data privacy: All data remains on local machine; no external API calls except optional analytics (opt-in)

5. Assumptions & Constraints
5.1: Ollama service must be pre-installed or provided via Docker
5.2: Single-user local deployment; multi-user scaling out of scope
5.3: Network dependency: None (fully offline after model download)
5.4: GPU acceleration: Optional (CUDA-enabled GPU recommended but not required)
5.5: Operating systems: Linux, macOS, Windows (via WSL2 or Docker Desktop)

6. Success Criteria
6.1: All functional requirements FR-01 through FR-05 implemented and tested
6.2: System runs stable on 8GB RAM for 4+ hours without memory leaks
6.3: RAG Q&A responds within 3-second SLA 95% of the time
6.4: User can upload, search, and query 100+ documents without system degradation

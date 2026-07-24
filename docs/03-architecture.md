Architecture Design Document
DocMind AI
1. Architectural Overview
DocMind follows a microservices-inspired polyglot architecture optimized for modularity, performance, and local execution on an 8GB RAM machine. Each service is chosen for its specific strengths in the technology stack:

TypeScript/React: Responsive client-side interface.

Go: Fast, lightweight API gateway and orchestrator.

Rust: High-performance text processing, parsing, and search indexing.

Python: Flexible AI/ML layer for embeddings and LLM inference.

PostgreSQL: Reliable relational storage for users, metadata, and document structures.

2. System Architecture Diagram
Plaintext
+-------------------------------------------------------------+
|                      Client Layer                           |
|                  TypeScript / React (UI)                    |
+-------------------------------------------------------------+
                               |
                               | HTTP / REST (JSON)
                               v
+-------------------------------------------------------------+
|                     API Gateway Layer                       |
|                        Go Backend                           |
|       - Authentication (JWT)  - Request Orchestration       |
+-------------------------------------------------------------+
             |                                   |
             | Document Ingestion                | Search / Q&A Requests
             v                                   v
+------------------------+             +----------------------+
|  Search Engine (Rust)  |             |  AI Service (Python) |
|  - Text Parsing        | <=========> |  - Embedding Gen.    |
|  - Indexing & Storage  |             |  - RAG / LLM Orchestration|
+------------------------+             +----------------------+
             ^                                   ^
             |                                   |
             +-----------------+-----------------+
                               |
                               v
+-------------------------------------------------------------+
|                      Persistence Layer                      |
|                        PostgreSQL                           |
|       - User Accounts   - File Metadata   - Chunk Index     |
+-------------------------------------------------------------+
3. Core Component Responsibilities
A. Frontend (TypeScript / React)
Renders the dashboard, document upload interface, search bar, and chat interface.

Communicates exclusively with the Go Backend API via RESTful endpoints.

B. Backend API (Go)
Acts as the single entry point (API Gateway) for the frontend.

Manages user authentication, session validation, and database interactions with PostgreSQL.

Orchestrates data flow by dispatching uploaded files to the Rust service for parsing and indexing, and calling the Python service for AI generation.

C. Search Engine Service (Rust)
Handles heavy text manipulation, tokenization, and document chunking.

Indexes document contents for high-speed retrieval.

Chosen for Rust's memory safety and zero-cost abstractions, ensuring minimal RAM consumption.

D. AI Service (Python)
Manages text vectorization (generating embeddings for chunks).

Handles Retrieval-Augmented Generation (RAG) by combining retrieved document chunks with a local LLM prompt.

Interfaces with lightweight, locally hosted models (e.g., via Ollama) to operate entirely offline within the 8GB RAM limit.

E. Database (PostgreSQL)
Stores relational entities: user credentials, project metadata, document records, and structural references.

4. Data Flow Scenarios
Scenario 1: Document Upload & Ingestion
User uploads a file (.txt, .md, or .pdf) via the TypeScript frontend.

Go Backend receives the file, saves metadata to PostgreSQL, and forwards the raw file data to the Rust service.

Rust Service parses the text, breaks it into logical chunks, and indexes it.

Concurrently or sequentially, the Python AI Service generates vector embeddings for the chunks, storing references back in the database or search index.

Scenario 2: AI Q&A Search (RAG Pipeline)
User submits a natural language question through the UI.

Go Backend receives the query and queries the Rust Search Service (and/or vector store) for the most relevant document chunks.

Go Backend passes the user question along with the retrieved chunks to the Python AI Service.

Python Service structures the context prompt, queries the local LLM, and streams/returns the final answer with source references back through Go to the Frontend.

5. Deployment & Local Infrastructure
Docker Compose: Manages container lifecycles for all components, ensuring isolated environments with controlled CPU and memory parameters suitable for an 8GB machine.

Environment Configuration: Managed via secure .env files injected at runtime.

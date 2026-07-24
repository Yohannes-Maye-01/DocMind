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
FR-01.1: The system must allow users to upload documents via the frontend interface.

FR-01.2: Supported initial file formats must include Plain Text (.txt), Markdown (.md), and Portable Document Format (.pdf).

FR-01.3: The backend must parse uploaded documents into manageable text chunks for indexing and vector embedding.

FR-01.4: Users must be able to view a list of all uploaded documents and delete documents when no longer needed.

FR-02: Search Engine Service (Rust)
FR-02.1: The Rust search service must index parsed document chunks for rapid retrieval.

FR-02.2: The system must support keyword/full-text search and prepare structured data payloads for vector similarity search.

FR-02.3: Search queries initiated from the frontend through the Go backend must return relevant document snippets within optimal local latency.

FR-03: AI Service & RAG Q&A (Python)
FR-03.1: The Python AI service must generate text embeddings for document chunks.

FR-03.2: The system must implement a Retrieval-Augmented Generation (RAG) pipeline: fetching relevant chunks via the search engine and injecting them into a local LLM prompt.

FR-03.3: The AI service must provide natural language answers to user queries accompanied by source document references.

FR-04: User Authentication & Security
FR-04.1: Users must be able to securely register an account and log in.

FR-04.2: Session management must be handled via secure tokens (e.g., JWT).

FR-04.3: Environment variables and secrets (database passwords, API configs) must be kept secure using .env files and excluded from version control.

4. Non-Functional Requirements & Constraints
NFR-01: Performance & Resource Efficiency (8GB RAM Constraint)
NFR-01.1: The entire containerized stack (Go, Rust, Python, PostgreSQL) must run comfortably within an 8GB RAM local environment using Docker Compose.

NFR-01.2: AI models used in the Python service must be lightweight or quantized (running locally via tools like Ollama or efficient Hugging Face models) to prevent memory exhaustion.

NFR-02: Modularity & Architecture
NFR-02.1: The system must maintain strict separation of concerns:

TypeScript/React: Frontend presentation and user interactions.

Go API: Core routing, authentication, and service orchestration.

Rust Service: High-performance indexing and text processing.

Python Service: ML, embedding, and LLM inference.

PostgreSQL: Relational data and metadata persistence.

NFR-03: Maintainability & Documentation
NFR-03.1: Codebases must follow professional standards for each respective language (idiomatic Go, clean Rust practices, PEP 8 Python style, and modular TypeScript components).

NFR-03.2: Architecture and development choices must be documented step-by-step in the docs/ folder.

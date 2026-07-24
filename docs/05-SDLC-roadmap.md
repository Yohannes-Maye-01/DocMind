# DocMind SDLC Roadmap

## Overview
This document outlines the realistic Software Development Life Cycle (SDLC) for the DocMind project. It's structured in 8 phases spanning approximately 18 weeks, with parallel development streams and clear deliverables.

---

## Phase 1: Foundation & Setup (Weeks 1-2)
**Primary Goal:** Establish all infrastructure, CI/CD pipelines, and development environment without implementing business logic.

### Tasks
- [ ] Initialize Git repository structure
  - [ ] Create `/backend` directory for Go
  - [ ] Create `/search-service` directory for Rust
  - [ ] Create `/ai-service` directory for Python
  - [ ] Create `/frontend` directory for TypeScript/React
  - [ ] Create `/docs` directory (already exists)
  - [ ] Create `.github/workflows` for CI/CD

- [ ] Set up Docker Compose
  - [ ] Define all 6 services (PostgreSQL, Go, Rust, Python, Ollama, Frontend)
  - [ ] Set memory limits (see NFR-01.1 in requirements)
  - [ ] Configure Docker internal network isolation
  - [ ] Create `.env.example` template

- [ ] Initialize CI/CD Pipeline (GitHub Actions)
  - [ ] Linting workflow (Go, Rust, Python, TypeScript)
  - [ ] Unit test workflow
  - [ ] Docker build workflow
  - [ ] Dependency security scanning

- [ ] Set up project scaffolding for each service
  - [ ] Go: Basic HTTP server template with health endpoint
  - [ ] Rust: Cargo project with basic HTTP server
  - [ ] Python: Virtual environment + FastAPI template
  - [ ] TypeScript: React + Vite scaffolding

- [ ] Create development documentation
  - [ ] Local setup guide (how to run services locally)
  - [ ] Environment variables reference
  - [ ] Code style guide per language
  - [ ] Git workflow & branch strategy

### Success Criteria
- [ ] All services run in Docker without errors
- [ ] GitHub Actions workflows execute successfully
- [ ] Developer can start entire system with `docker-compose up`
- [ ] All services respond to `/health` endpoint

### Deliverables
- Complete `docker-compose.yml`
- CI/CD workflow files (`.github/workflows/*.yml`)
- `.env.example` file
- Development setup guide
- Service scaffolding (skeleton projects)

---

## Phase 2: Database & API Contract (Weeks 3-4)
**Primary Goal:** Define the contract between services and establish persistence layer.

### Tasks

#### Database Design & Migrations
- [ ] Create PostgreSQL initialization scripts
  - [ ] User table with authentication fields
  - [ ] Documents table with metadata
  - [ ] Document chunks table
  - [ ] Add indexes for performance
  - [ ] Add foreign key constraints

- [ ] Set up database migrations tool
  - [ ] Use Flyway or sql-migrate
  - [ ] Create versioned migration files
  - [ ] Test migration rollback

#### Go Backend - Core Layer
- [ ] Database connection setup
  - [ ] Connection pooling (pgxpool, max 5 connections)
  - [ ] Health check for database connectivity

- [ ] Create API contract (OpenAPI/Swagger)
  - [ ] Document all endpoints (planned, not just current)
  - [ ] Define request/response schemas
  - [ ] Generate Swagger UI endpoint

- [ ] Implement middleware
  - [ ] Request logging
  - [ ] Error handling middleware
  - [ ] CORS middleware (restrict to localhost:3000)
  - [ ] Authentication middleware (prepare for Phase 3)

- [ ] Create service health check endpoints
  - [ ] `/health` - Overall system health
  - [ ] `/health/database` - Database connectivity
  - [ ] `/health/services` - Status of Rust & Python services

#### Testing Setup
- [ ] Unit test framework configuration (Go)
- [ ] Integration test configuration
- [ ] Mock database setup for tests

### Success Criteria
- [ ] PostgreSQL schema matches `04-database-design.md`
- [ ] Database migrations work forward and backward
- [ ] All Go health endpoints respond correctly
- [ ] OpenAPI spec is complete and accurate
- [ ] GitHub Actions runs database tests

### Deliverables
- PostgreSQL migration scripts (`/backend/migrations/`)
- OpenAPI/Swagger specification (`docs/openapi.yaml`)
- Go Backend with database layer
- Health check endpoints working
- API documentation site (Swagger UI accessible)

---

## Phase 3: Core Features - MVP (Weeks 5-8)
**Primary Goal:** Implement document management and search functionality. Parallel work across all services.

### 3A: Document Management (Go Backend + Rust Service + PostgreSQL)

#### Go Backend - Document Endpoints
- [ ] `POST /api/documents/upload` - File upload endpoint
  - [ ] Accept .txt, .md, .pdf files (max 50MB)
  - [ ] Store file metadata in PostgreSQL
  - [ ] Call Rust service for parsing & indexing
  - [ ] Return document ID to frontend

- [ ] `GET /api/documents` - List all documents
  - [ ] Pagination support
  - [ ] Return document metadata (title, size, date, status)

- [ ] `GET /api/documents/{id}` - Get single document details
  - [ ] Include chunk count
  - [ ] Include indexing status

- [ ] `DELETE /api/documents/{id}` - Delete document
  - [ ] Remove from PostgreSQL
  - [ ] Remove from Rust index

#### Rust Service - Text Processing
- [ ] Text parser for multiple formats
  - [ ] Plain text (.txt) parser
  - [ ] Markdown (.md) parser
  - [ ] PDF parser (using `pdfium-render` or similar crate)

- [ ] Document chunking logic
  - [ ] 512-token sliding window
  - [ ] 50-token overlap
  - [ ] Store chunks in PostgreSQL via Go Backend

- [ ] In-memory indexing
  - [ ] BTreeMap-based index structure
  - [ ] Store mapping of tokens to chunk IDs

- [ ] HTTP endpoints
  - [ ] `POST /parse` - Accept raw file, return parsed text
  - [ ] `POST /index` - Accept parsed text, index it, return chunk list
  - [ ] `GET /health` - Service health check

#### PostgreSQL
- [ ] Implement document_chunks table storage
- [ ] Create queries for chunk retrieval

#### Frontend (TypeScript/React)
- [ ] Document upload form
  - [ ] Drag-and-drop support
  - [ ] File type validation
  - [ ] Progress indicator

- [ ] Document list view
  - [ ] Display all uploaded documents
  - [ ] Show file metadata (name, size, upload date, status)
  - [ ] Delete button with confirmation

- [ ] Basic styling
  - [ ] Use TailwindCSS or Bootstrap
  - [ ] Responsive design for desktop/tablet

#### Testing
- [ ] Unit tests: Rust parsing logic (80% coverage target)
- [ ] Unit tests: Go API endpoints (70% coverage target)
- [ ] Integration test: Full upload flow (file → parse → index → database)
- [ ] Component tests: React upload & list components

### 3B: Search Engine (Rust Service - Enhanced)

#### Rust Service - Search Implementation
- [ ] Keyword/full-text search
  - [ ] Tokenization logic
  - [ ] Query parsing
  - [ ] Ranking algorithm (TF-IDF or BM25)

- [ ] Relevance scoring
  - [ ] Score calculation based on token frequency
  - [ ] Return ranked results

- [ ] HTTP endpoints
  - [ ] `POST /search` - Search query endpoint
    - [ ] Request: `{"query": "...", "page": 1, "limit": 20}`
    - [ ] Response: `{"results": [...], "total": N, "page": 1}`

- [ ] Pagination support
  - [ ] 20 results per page
  - [ ] Total count in response

#### Go Backend - Search Orchestration
- [ ] `GET /api/search?q=...&page=...` endpoint
  - [ ] Call Rust search service
  - [ ] Return results with source document references

#### Frontend
- [ ] Search bar component
- [ ] Search results display
  - [ ] Show snippet of matching text
  - [ ] Show source document
  - [ ] Highlight matching terms

#### Testing
- [ ] Unit tests: Search algorithm (80% coverage)
- [ ] Integration test: Search with 100+ documents
- [ ] Performance test: Search < 500ms SLA

### Success Criteria for Phase 3
- [ ] Upload a document → see it in list ✅
- [ ] Search documents → get relevant results ✅
- [ ] Delete document → removed from system ✅
- [ ] All services health checks pass ✅
- [ ] Frontend UI is functional and responsive ✅
- [ ] Test coverage: Go 70%, Rust 80%, TypeScript 60%

### Deliverables
- Complete document management flow (end-to-end)
- Full-text search working
- All unit & integration tests passing
- Frontend document management UI

---

## Phase 4: AI Integration (Weeks 9-11)
**Primary Goal:** Add intelligence layer with embeddings and RAG pipeline.

### Python AI Service - Setup
- [ ] FastAPI service scaffold
- [ ] `/health` endpoint
- [ ] Logging and error handling

### Embedding Generation (Python)
- [ ] Model loading
  - [ ] Download Sentence Transformers 'all-MiniLM-L6-v2' (~85MB)
  - [ ] Load model into memory on service startup

- [ ] HTTP endpoints
  - [ ] `POST /embed` - Generate embeddings
    - [ ] Request: `{"texts": [...]}`
    - [ ] Response: `{"embeddings": [[...], [...]], ...}`

- [ ] Vector storage interface
  - [ ] Store embeddings in PostgreSQL (optional) or in-memory
  - [ ] Retrieve embeddings for similarity search

### Rust Service - Vector Search Enhancement
- [ ] Prepare vector payloads
  - [ ] Format chunk data for vector search
  - [ ] Optional: Implement cosine similarity calculation

### Ollama Integration (Python)
- [ ] Ollama service (runs in separate Docker container)
  - [ ] Use Mistral 7B Q4 (~5GB) or Phi 2 Q4 (~3GB)
  - [ ] Configure resource limits

- [ ] LLM query interface
  - [ ] `POST /query-llm` endpoint
    - [ ] Request: `{"prompt": "...", "context": "..."}`
    - [ ] Response: `{"answer": "..."}`

### RAG Pipeline (Python)
- [ ] `POST /rag-query` endpoint
  - [ ] Accept user question
  - [ ] Call Go Backend to search relevant chunks
  - [ ] Generate embeddings for question
  - [ ] Retrieve top 5 relevant chunks
  - [ ] Structure RAG prompt with context
  - [ ] Query Ollama LLM
  - [ ] Return answer with source references

### Go Backend - Q&A Orchestration
- [ ] `POST /api/query` endpoint
  - [ ] Accept user question
  - [ ] Call Rust search service
  - [ ] Call Python RAG service
  - [ ] Return answer to frontend

### Frontend - Chat Interface
- [ ] Chat/Q&A interface
  - [ ] Input field for questions
  - [ ] Display AI response
  - [ ] Show source document references
  - [ ] Conversation history (optional for MVP)

### Testing
- [ ] Unit tests: Embedding generation (75% coverage)
- [ ] Unit tests: RAG pipeline (75% coverage)
- [ ] Integration test: Full Q&A flow (question → search → embed → LLM → answer)
- [ ] Load test: 10 concurrent RAG queries
- [ ] Latency test: Q&A response < 3 seconds

### Success Criteria
- [ ] User asks a question → receives AI answer ✅
- [ ] Answer includes source document references ✅
- [ ] Response time < 3 seconds ✅
- [ ] Multiple concurrent queries handled ✅

### Deliverables
- Python AI service with embeddings & RAG
- Ollama integration working
- Frontend chat interface
- End-to-end RAG pipeline tested

---

## Phase 5: Optimization & Refinement (Weeks 12-14)
**Primary Goal:** Stabilize system, meet SLAs, optimize for 8GB RAM constraint.

### Performance Profiling
- [ ] Profile each service
  - [ ] Memory usage baseline (Go, Rust, Python, Ollama)
  - [ ] CPU usage under load
  - [ ] Disk I/O patterns

- [ ] Identify bottlenecks
  - [ ] Search latency analysis
  - [ ] Embedding generation time
  - [ ] LLM inference time
  - [ ] Database query times

### Memory Optimization
- [ ] Verify memory allocation per NFR-01.1
  - [ ] PostgreSQL: 500MB ✅
  - [ ] Go API: 200MB ✅
  - [ ] Rust Service: 300MB ✅
  - [ ] Python Service (embeddings): 1.5GB ✅
  - [ ] Python Service (LLM via Ollama): 3.5GB ✅
  - [ ] Frontend/System: ~1GB ✅
  - [ ] Headroom: ~500MB ✅

- [ ] Optimize if over budget
  - [ ] Connection pooling tuning
  - [ ] Cache strategies
  - [ ] Memory leak detection

### Latency Optimization
- [ ] Search: Target < 500ms
  - [ ] Index optimization
  - [ ] Query caching
  - [ ] Parallel search if needed

- [ ] RAG Q&A: Target < 3 seconds
  - [ ] Embedding caching
  - [ ] LLM prompt optimization
  - [ ] Connection pooling improvements

- [ ] Document upload: Target < 10 seconds for 10MB PDF
  - [ ] Chunking optimization
  - [ ] Indexing parallelization

### Error Handling & Resilience
- [ ] Implement retry logic
  - [ ] Exponential backoff (1s, 2s, 4s, 8s with jitter)
  - [ ] Max 3 retries per request

- [ ] Graceful degradation
  - [ ] If Rust service down: return error with clear message
  - [ ] If Python service down: return error with clear message
  - [ ] If Ollama down: return error, suggest restart

- [ ] Circuit breaker pattern
  - [ ] Prevent cascading failures
  - [ ] Auto-recovery mechanism

### Data Management
- [ ] Chunk size validation
  - [ ] Ensure 512-token window + 50-token overlap
  - [ ] Test with various document formats

- [ ] Index consistency
  - [ ] Verify all chunks indexed
  - [ ] Handle partial indexing failures

### Testing
- [ ] Load test: 100 concurrent single-user operations
- [ ] Stress test: Sustained load for 1 hour
- [ ] Memory test: Monitor for leaks over 4+ hours
- [ ] Latency benchmarks documented

### Success Criteria
- [ ] Memory usage stays within budget ✅
- [ ] Search latency < 500ms (95th percentile) ✅
- [ ] RAG response < 3 seconds (95th percentile) ✅
- [ ] System stable for 4+ hours without degradation ✅

### Deliverables
- Performance baseline report
- Memory profiling report
- Optimization recommendations implemented
- Load test results

---

## Phase 6: Testing & Quality Assurance (Weeks 15-16)
**Primary Goal:** Comprehensive test coverage, security validation, and complete documentation.

### Unit Test Coverage
- [ ] Go Backend
  - [ ] API endpoint tests
  - [ ] Database layer tests
  - [ ] Service orchestration tests
  - [ ] Target: 70% coverage

- [ ] Rust Service
  - [ ] Text parsing tests (all formats: .txt, .md, .pdf)
  - [ ] Chunking tests (verify 512-token window, 50-token overlap)
  - [ ] Search algorithm tests
  - [ ] Indexing tests
  - [ ] Target: 80% coverage

- [ ] Python Service
  - [ ] Embedding generation tests
  - [ ] RAG pipeline tests
  - [ ] LLM integration tests
  - [ ] Error handling tests
  - [ ] Target: 75% coverage

- [ ] TypeScript Frontend
  - [ ] Component tests (upload, list, search, chat)
  - [ ] Form validation tests
  - [ ] API integration tests
  - [ ] Target: 60% coverage

### Integration Tests
- [ ] End-to-end document lifecycle
  - [ ] Upload → Index → Search → Delete

- [ ] Multi-document scenarios
  - [ ] Search across 10+ documents
  - [ ] Q&A with multiple relevant sources

- [ ] Error scenarios
  - [ ] Upload unsupported file type
  - [ ] Search with empty query
  - [ ] Delete non-existent document
  - [ ] Service unavailability handling

- [ ] Concurrency tests
  - [ ] Concurrent uploads
  - [ ] Concurrent searches
  - [ ] Concurrent Q&A queries

### Security Testing
- [ ] SQL Injection prevention
  - [ ] Test parameterized queries
  - [ ] Verify input validation

- [ ] JWT Authentication (if implemented)
  - [ ] Token validation
  - [ ] Token expiration
  - [ ] Refresh token rotation

- [ ] Data validation
  - [ ] File type validation
  - [ ] File size limits enforced
  - [ ] Query parameter sanitization

- [ ] CORS & network isolation
  - [ ] Verify localhost:3000 restriction
  - [ ] Test internal Docker network isolation

### Documentation
- [ ] API Reference
  - [ ] OpenAPI/Swagger (auto-generated if possible)
  - [ ] Endpoint examples with curl/Postman
  - [ ] Error codes and responses

- [ ] Architecture Diagrams
  - [ ] Data flow diagrams (Mermaid)
  - [ ] Service interaction diagrams
  - [ ] Component dependency graph

- [ ] Deployment Guide
  - [ ] Step-by-step setup instructions
  - [ ] Environment variable reference
  - [ ] Docker Compose explanation

- [ ] Troubleshooting Guide
  - [ ] Common issues and solutions
  - [ ] Debug mode instructions
  - [ ] Log inspection tips

- [ ] API Endpoint Reference
  - [ ] All endpoints documented
  - [ ] Request/response examples
  - [ ] Authentication requirements

### Success Criteria
- [ ] Unit test coverage: Go 70%, Rust 80%, Python 75%, TS 60% ✅
- [ ] All integration tests pass ✅
- [ ] Security tests pass (no vulnerabilities) ✅
- [ ] Documentation complete and accurate ✅

### Deliverables
- Test coverage reports (SonarQube or similar)
- Security audit checklist (completed)
- Complete API documentation
- Architecture documentation
- Deployment guide
- Troubleshooting guide

---

## Phase 7: Deployment & DevOps (Weeks 17-18)
**Primary Goal:** Automate everything; production-ready local deployment.

### Docker & Compose Finalization
- [ ] Optimize Dockerfile for each service
  - [ ] Multi-stage builds to reduce image size
  - [ ] Security best practices (non-root user, minimal base image)

- [ ] Docker Compose v3 optimization
  - [ ] Resource limits and reservations
  - [ ] Health checks for all services
  - [ ] Restart policies
  - [ ] Volume management for persistence

- [ ] Environment configuration
  - [ ] Finalize `.env.example`
  - [ ] Document all variables
  - [ ] Secrets management best practices

### CI/CD Pipeline (GitHub Actions)
- [ ] Automated testing workflow
  - [ ] Trigger on PR
  - [ ] Run all unit tests
  - [ ] Check code coverage
  - [ ] Enforce minimum coverage threshold

- [ ] Code quality checks
  - [ ] Linting (Go: golangci-lint, Rust: clippy, Python: pylint/flake8, TS: ESLint)
  - [ ] Formatting (Go: gofmt, Rust: rustfmt, Python: black, TS: Prettier)
  - [ ] Code analysis (SonarQube or CodeFactor)

- [ ] Security scanning
  - [ ] Dependency vulnerability scanning (Snyk, Dependabot)
  - [ ] SAST scanning (SonarQube)
  - [ ] Container image scanning

- [ ] Build & push workflow
  - [ ] Build Docker images
  - [ ] Run integration tests in containers
  - [ ] (Optional) Push to Docker registry

- [ ] Performance testing workflow
  - [ ] Run load tests on tagged releases
  - [ ] Compare against baselines

### Deployment Automation
- [ ] One-command deployment script
  - [ ] `./deploy.sh` or similar
  - [ ] Handles:
    - [ ] Docker image build
    - [ ] Database migration
    - [ ] Service startup
    - [ ] Health check

- [ ] Rollback procedure
  - [ ] Document rollback steps
  - [ ] Test rollback mechanism

### Monitoring & Logging (Optional for MVP)
- [ ] Centralized logging
  - [ ] All services log to stdout (Docker handles aggregation)
  - [ ] Structured logging (JSON format)
  - [ ] Log levels: ERROR, WARN, INFO, DEBUG

- [ ] Health monitoring
  - [ ] Dashboard showing service health
  - [ ] Alerts for service failures (optional)

- [ ] Performance metrics
  - [ ] Track latency percentiles (p50, p95, p99)
  - [ ] Memory usage trends
  - [ ] Query throughput

### Success Criteria
- [ ] `docker-compose up` starts entire system ✅
- [ ] All GitHub Actions workflows pass ✅
- [ ] Deployment script works on clean system ✅
- [ ] CI/CD executes on every PR ✅

### Deliverables
- Optimized Dockerfile for all services
- Production-ready docker-compose.yml
- Complete GitHub Actions CI/CD pipeline
- One-command deployment script
- Monitoring setup (if implemented)

---

## Phase 8: Maintenance & Iteration (Ongoing)
**Primary Goal:** Support production system with continuous improvements.

### Ongoing Tasks
- [ ] Bug fixes and hotfixes
  - [ ] Create hotfix branch from main
  - [ ] Test and merge quickly
  - [ ] Deploy and monitor

- [ ] Performance tuning
  - [ ] Monitor metrics continuously
  - [ ] Adjust resource allocation as needed
  - [ ] Optimize slow queries

- [ ] Dependency updates
  - [ ] Monthly dependency security updates
  - [ ] Major version updates quarterly

- [ ] Feature requests
  - [ ] Document in GitHub issues
  - [ ] Prioritize for future phases

- [ ] User feedback incorporation
  - [ ] Collect feedback from usage
  - [ ] Iterate on UI/UX
  - [ ] Add missing features

### Future Phases (Out of MVP Scope)
- [ ] Multi-user support
- [ ] Advanced search filters
- [ ] Document versioning
- [ ] Collaborative features
- [ ] API for external integrations
- [ ] Web-based deployment (beyond local)

---

## Success Metrics & Checkpoints

| Phase | Key Metric | Target | Status |
|-------|-----------|--------|--------|
| 1 | System boots with `docker-compose up` | ✅ 100% | ⬜ |
| 2 | API spec complete & DB migrations working | ✅ 100% | ⬜ |
| 3 | Upload → Search end-to-end | ✅ 100% | ⬜ |
| 4 | RAG Q&A working with < 3s latency | ✅ 95% of requests | ⬜ |
| 5 | Memory under 8GB, all SLAs met | ✅ 100% | ⬜ |
| 6 | Test coverage targets met | ✅ 100% | ⬜ |
| 7 | Full CI/CD pipeline automated | ✅ 100% | ⬜ |
| 8 | System production-ready | ✅ 100% | ⬜ |

---

## Risk Mitigation

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|-----------|
| 8GB RAM constraint violated | Medium | High | Phase 5 profiling & optimization |
| Ollama LLM too slow | Medium | Medium | Test with Phi 2 (faster than Mistral) |
| PDF parsing failures | Low | Medium | Comprehensive testing, fallback to text extraction |
| Service coupling issues | Low | High | Clear API contracts in Phase 2 |
| Learning curve delays | High | Medium | Allocate +20% buffer per phase |
| Database query performance | Medium | High | Indexing strategy, query optimization in Phase 5 |

---

## Notes for Team/Self

1. **Parallel Work:** After Phase 2, Rust, Go, and Python services can develop in parallel.
2. **Testing First:** Write tests as you go—don't defer to Phase 6.
3. **Documentation:** Update docs incrementally, not at the end.
4. **Learning:** Expect 20% schedule buffer due to learning curve (you mentioned you're learning these languages).
5. **Communication:** Use GitHub Issues/Projects to track progress publicly.
6. **Code Review:** Even solo, enable branch protection and PR reviews for quality.

---

## Getting Started (Next Steps)

1. ✅ Read this roadmap thoroughly
2. ⬜ Create GitHub Issues for each Phase (use templates)
3. ⬜ Set up a GitHub Project board (Kanban view)
4. ⬜ Start Phase 1: Foundation & Setup
5. ⬜ Commit progress daily
6. ⬜ Update this roadmap as you discover new insights

Good luck building DocMind! 🚀

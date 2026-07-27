# 🧠 DocMind

> AI-powered document intelligence platform — upload files, search content, and ask questions using a fully offline RAG pipeline.

[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Go](https://img.shields.io/badge/Go-1.22-00ADD8)](https://golang.org/)
[![Rust](https://img.shields.io/badge/Rust-1.79-orange)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.11-blue)](https://www.python.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5-3178C6)](https://www.typescriptlang.org/)

---

## Architecture

```
┌─────────────────────────────────────────┐
│        Frontend  · TypeScript/React     │  :3000
└───────────────────┬─────────────────────┘
                    │ HTTP/REST
┌───────────────────▼─────────────────────┐
│       Backend API · Go (chi)            │  :8080
│   Auth · Routing · Orchestration        │
└──────────┬────────────────┬─────────────┘
           │                │
┌──────────▼──────┐  ┌──────▼──────────────┐
│ Search Service  │  │    AI Service        │
│  Rust + axum    │  │  Python + FastAPI    │  :8081 / :8082
│  BTreeMap index │  │  Embeddings + RAG    │
└──────────┬──────┘  └──────┬──────────────┘
           │                │
┌──────────▼────────────────▼─────────────┐
│          PostgreSQL 16                   │  :5432
└──────────────────────────────────────────┘
           ↕ LLM inference
┌─────────────────────────────────────────┐
│          Ollama (local LLM)              │  :11434
│     phi:2 Q4 or mistral:7b Q4           │
└─────────────────────────────────────────┘
```

**Memory budget (8GB machine):** PostgreSQL 500MB · Go 200MB · Rust 300MB · Python 1.5GB · Ollama 3.5GB · System ~1GB

---
 
## Project Structure

```
DocMind/
├── backend/                  # Go REST API & orchestrator
│   ├── cmd/server/           # Entry point
│   ├── internal/
│   │   ├── api/              # Handlers, middleware, router
│   │   ├── config/           # Env var loading
│   │   └── database/         # pgxpool + SQL migrations
│   └── Dockerfile
├── search-service/           # Rust text processing & search
│   ├── src/
│   │   ├── api/              # axum routes
│   │   ├── index/            # BTreeMap inverted index
│   │   ├── parser/           # .txt / .md / .pdf → chunks
│   │   └── search/           # TF-IDF ranking
│   └── Dockerfile
├── ai-service/               # Python embeddings & RAG
│   ├── app/
│   │   ├── api/routes/       # /embed  /rag/query
│   │   ├── services/         # SentenceTransformer + Ollama
│   │   └── core/             # Pydantic settings
│   └── Dockerfile
├── frontend/                 # TypeScript / React SPA
│   ├── src/
│   │   ├── api/              # Axios client
│   │   ├── components/       # UI components (Phase 3)
│   │   ├── pages/            # Route pages (Phase 3)
│   │   └── types/            # Shared TS interfaces
│   └── Dockerfile
├── docs/                     # SDLC documentation
├── scripts/                  # setup.sh  deploy.sh
├── docker-compose.yml        # Full stack orchestration
├── .env.example              # Environment template
└── Makefile                  # Developer shortcuts
```

---

## Quick Start

```bash
# 1. Clone and enter
git clone https://github.com/Yohannes-Maye-01/DocMind.git && cd DocMind

# 2. Create your .env
cp .env.example .env        # then edit with real secrets

# 3. Build and start everything
make up-build

# 4. Pull the local LLM (first time only, ~3GB)
make pull-model

# 5. Open the app
open http://localhost:3000
```

---

## Developer Commands

| Command | Description |
|---------|-------------|
| `make up` | Start all services (detached) |
| `make up-build` | Rebuild images + start |
| `make down` | Stop all services |
| `make logs` | Tail all logs |
| `make test` | Run all tests |
| `make lint` | Run all linters |
| `make db-shell` | Open PostgreSQL shell |

See `Makefile` for the full list.

---

## Service READMEs

- [Backend (Go)](./backend/README.md)
- [Search Service (Rust)](./search-service/README.md)
- [AI Service (Python)](./ai-service/README.md)
- [Frontend (TypeScript)](./frontend/README.md)

---

## SDLC Status

See [`docs/05-SDLC-roadmap.md`](./docs/05-SDLC-roadmap.md) for the full 18-week plan.

| Phase | Description | Status |
|-------|-------------|--------|
| 1 | Foundation & Setup | 🟡 In Progress |
| 2 | Database & API Contract | ⬜ Pending |
| 3 | Core Features (MVP) | ⬜ Pending |
| 4 | AI Integration | ⬜ Pending |
| 5 | Optimization | ⬜ Pending |
| 6 | Testing & QA | ⬜ Pending |
| 7 | Deployment & DevOps | ⬜ Pending |
| 8 | Maintenance | ⬜ Ongoing |

---

## License

MIT © DocMind Team

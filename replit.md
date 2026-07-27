# DocMind – Project Overview

AI-powered document intelligence platform. Upload files, run semantic search, and ask natural language questions via a fully offline RAG pipeline.

## Stack
- **Frontend:** TypeScript + React 18 + Vite (port 3000)
- **Backend:** Go 1.22 + chi router (port 8080)
- **Search Service:** Rust 1.79 + axum (port 8081)
- **AI Service:** Python 3.11 + FastAPI (port 8082)
- **Database:** PostgreSQL 16 (port 5432)
- **LLM:** Ollama – phi:2 or mistral:7b-q4 (port 11434)

## Folder Structure
```
backend/          Go API gateway
search-service/   Rust text processing & BTreeMap index
ai-service/       Python embeddings (SentenceTransformers) + RAG
frontend/         React SPA
docs/             SDLC documentation (requirements, architecture, DB design)
scripts/          setup.sh  deploy.sh
```

## How to Run (Docker)
```bash
cp .env.example .env   # fill in secrets
make up-build          # build + start all services
make pull-model        # pull Ollama phi:2 (~3GB, first time only)
```

## Key Files
- `docker-compose.yml` — full stack orchestration
- `.env.example` — environment variable template
- `Makefile` — developer shortcuts (make help)
- `docs/AI_CONTEXT.md` — project goals and AI assistant rules

## User Preferences
- Step-by-step explanations preferred
- Follow SDLC phases strictly
- 8GB RAM machine — keep memory budgets per NFR-01.1
- Prefer free/open-source tools
- Fully offline after initial model download

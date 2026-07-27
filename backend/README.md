# DocMind – Backend (Go)

REST API gateway built with Go. Authenticates requests, orchestrates calls to the Rust search service and Python AI service, and persists data to PostgreSQL.

## Stack
- **Language:** Go 1.22
- **Router:** chi v5
- **Database driver:** pgx v5 (pgxpool)
- **Auth:** JWT (golang-jwt/jwt v5)

## Structure
```
backend/
├── cmd/server/          # Entry point (main.go)
├── internal/
│   ├── api/
│   │   ├── handlers/    # HTTP handler functions
│   │   ├── middleware/  # Structured logger, auth, etc.
│   │   └── router.go    # Route registration
│   ├── config/          # Env var loading
│   └── database/
│       ├── migrations/  # SQL migration files (run at startup)
│       └── database.go  # pgxpool connection helper
├── go.mod
└── Dockerfile
```

## Running locally
```bash
# Set required env vars (copy from .env.example)
export POSTGRES_DSN="postgres://docmind:password@localhost:5432/docmind_db"
export JWT_SECRET="at-least-32-chars-secret"

go run ./cmd/server
# → listening on :8080
```

## API Endpoints
| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Overall health |
| GET | /health/database | DB connectivity |
| GET | /health/services | Downstream services |
| GET | /api/documents | List documents |
| POST | /api/documents/upload | Upload a document |
| GET | /api/documents/{id} | Get document details |
| DELETE | /api/documents/{id} | Delete a document |
| GET | /api/search?q= | Keyword/semantic search |
| POST | /api/query | RAG Q&A query |

## Code Style
- `gofmt` before every commit
- `go vet ./...` must pass
- 70% unit test coverage target (Phase 6)

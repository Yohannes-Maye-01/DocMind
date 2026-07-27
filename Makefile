# ─────────────────────────────────────────────────
#  DocMind – Developer Makefile
#  Usage: make <target>
# ─────────────────────────────────────────────────

.PHONY: help up down build logs clean setup lint test

## Default target
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ── Docker ─────────────────────────────────────
up: ## Start all services (detached)
	docker compose up -d

up-build: ## Build images and start all services
	docker compose up -d --build

down: ## Stop all services
	docker compose down

down-v: ## Stop all services and remove volumes
	docker compose down -v

build: ## Build all Docker images
	docker compose build

logs: ## Tail logs for all services
	docker compose logs -f

logs-backend: ## Tail backend logs
	docker compose logs -f backend

logs-search: ## Tail search service logs
	docker compose logs -f search-service

logs-ai: ## Tail AI service logs
	docker compose logs -f ai-service

# ── Setup ───────────────────────────────────────
setup: ## Copy .env.example → .env (if not exists)
	@[ -f .env ] && echo ".env already exists." || (cp .env.example .env && echo "Created .env from template.")

pull-model: ## Pull default Ollama model (phi:2)
	docker compose exec ollama ollama pull phi:2

# ── Individual services (local, no Docker) ──────
run-backend: ## Run Go backend locally
	cd backend && go run ./cmd/server

run-search: ## Run Rust search service locally
	cd search-service && cargo run

run-ai: ## Run Python AI service locally
	cd ai-service && uvicorn app.main:app --reload --port 8082

run-frontend: ## Run frontend dev server locally
	cd frontend && npm run dev

# ── Lint ────────────────────────────────────────
lint: lint-go lint-rust lint-python lint-ts ## Run all linters

lint-go: ## Lint Go code
	cd backend && gofmt -l . && go vet ./...

lint-rust: ## Lint Rust code
	cd search-service && cargo clippy -- -D warnings

lint-python: ## Lint Python code
	cd ai-service && ruff check app/ && mypy app/

lint-ts: ## Lint TypeScript code
	cd frontend && npm run lint

# ── Test ────────────────────────────────────────
test: test-go test-rust test-python test-ts ## Run all tests

test-go: ## Run Go tests
	cd backend && go test ./... -v

test-rust: ## Run Rust tests
	cd search-service && cargo test

test-python: ## Run Python tests
	cd ai-service && pytest tests/ -v

test-ts: ## Run TypeScript/React tests
	cd frontend && npm test

# ── Clean ───────────────────────────────────────
clean: ## Remove build artifacts
	cd backend && go clean
	cd search-service && cargo clean
	cd frontend && rm -rf dist node_modules/.cache

# ── Database ────────────────────────────────────
db-migrate: ## Apply database migrations
	docker compose exec postgres psql -U $${POSTGRES_USER} -d $${POSTGRES_DB} \
	  -f /docker-entrypoint-initdb.d/001_init.sql

db-shell: ## Open a PostgreSQL shell
	docker compose exec postgres psql -U $${POSTGRES_USER} -d $${POSTGRES_DB}

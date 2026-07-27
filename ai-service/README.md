# DocMind – AI Service (Python)

Embedding generation and RAG (Retrieval-Augmented Generation) pipeline. Interfaces with a local Ollama LLM.

## Stack
- **Language:** Python 3.11
- **Framework:** FastAPI + uvicorn
- **Embeddings:** sentence-transformers (`all-MiniLM-L6-v2`, ~85MB)
- **LLM:** Ollama (`phi:2` or `mistral:7b-q4`)

## Structure
```
ai-service/
├── app/
│   ├── api/
│   │   └── routes/
│   │       ├── embeddings.py   # POST /embed
│   │       └── rag.py          # POST /rag/query
│   ├── core/
│   │   └── config.py           # Pydantic settings
│   ├── services/
│   │   ├── embeddings.py       # SentenceTransformer wrapper
│   │   ├── llm.py              # Ollama HTTP client
│   │   └── rag.py              # RAG pipeline
│   ├── models/
│   │   └── schemas.py          # Shared Pydantic schemas
│   └── main.py                 # FastAPI app + lifespan
├── tests/
├── requirements.txt
└── Dockerfile
```

## Running locally
```bash
pip install -r requirements.txt
OLLAMA_URL=http://localhost:11434 uvicorn app.main:app --reload --port 8082
```

## Endpoints
| Method | Path | Description |
|--------|------|-------------|
| GET | /health | Service health |
| POST | /embed | Generate text embeddings |
| POST | /rag/query | RAG Q&A with local LLM |

## Code Style
- PEP 8 + type hints on all functions
- `ruff check app/` and `mypy app/` must pass
- 75% test coverage target (Phase 6)

"""DocMind AI Service – FastAPI entry point."""

from contextlib import asynccontextmanager

from fastapi import FastAPI

from app.api.routes import embeddings, rag
from app.core.config import settings


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Load the embedding model once at startup."""
    from app.services.embeddings import EmbeddingService
    app.state.embedder = EmbeddingService(model_name=settings.embedding_model)
    yield
    # cleanup (if needed)


app = FastAPI(
    title="DocMind AI Service",
    version="0.1.0",
    description="Embedding generation and RAG pipeline for DocMind.",
    lifespan=lifespan,
)

# ── Health ──────────────────────────────────────
@app.get("/health", tags=["health"])
async def health() -> dict:
    return {"status": "ok"}


# ── Routers ─────────────────────────────────────
app.include_router(embeddings.router, prefix="/embed",    tags=["embeddings"])
app.include_router(rag.router,        prefix="/rag",      tags=["rag"])

"""POST /rag/query – RAG Q&A pipeline."""

from fastapi import APIRouter, HTTPException, Request
from pydantic import BaseModel

from app.services.rag import RagService
from app.core.config import settings

router = APIRouter()


class RagRequest(BaseModel):
    question: str
    chunks: list[str]  # top-k chunks supplied by Go backend


class RagResponse(BaseModel):
    answer: str
    sources: list[str]


@router.post("/query", response_model=RagResponse)
async def rag_query(req: RagRequest, request: Request) -> RagResponse:
    """
    Receive a user question + relevant document chunks from the Go backend,
    build a RAG prompt, query the local LLM via Ollama, and return the answer.
    """
    if not req.chunks:
        raise HTTPException(status_code=400, detail="No context chunks provided.")

    embedder = request.app.state.embedder
    svc = RagService(
        embedder=embedder,
        ollama_url=settings.ollama_url,
        model=settings.ollama_model,
    )
    answer = await svc.answer(req.question, req.chunks)
    return RagResponse(answer=answer, sources=req.chunks[: settings.rag_top_k])

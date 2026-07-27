"""RAG pipeline: context injection + LLM query."""

from __future__ import annotations

from app.services.embeddings import EmbeddingService
from app.services.llm import OllamaClient

_PROMPT_TEMPLATE = """\
You are DocMind, an AI assistant. Answer the question using ONLY the context below.
If the answer is not in the context, say "I don't know based on the provided documents."

Context:
{context}

Question: {question}

Answer:"""


class RagService:
    def __init__(self, embedder: EmbeddingService, ollama_url: str, model: str) -> None:
        self._embedder = embedder
        self._llm = OllamaClient(base_url=ollama_url, model=model)

    async def answer(self, question: str, chunks: list[str]) -> str:
        context = "\n\n---\n\n".join(chunks)
        prompt = _PROMPT_TEMPLATE.format(context=context, question=question)
        return await self._llm.generate(prompt)

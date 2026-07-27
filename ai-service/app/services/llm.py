"""Thin async client for the Ollama local LLM API."""

from __future__ import annotations

import httpx


class OllamaClient:
    """Calls the Ollama /api/generate endpoint."""

    def __init__(self, base_url: str, model: str) -> None:
        self._url = f"{base_url}/api/generate"
        self._model = model

    async def generate(self, prompt: str) -> str:
        payload = {
            "model": self._model,
            "prompt": prompt,
            "stream": False,
        }
        async with httpx.AsyncClient(timeout=60.0) as client:
            resp = await client.post(self._url, json=payload)
            resp.raise_for_status()
            return resp.json().get("response", "")

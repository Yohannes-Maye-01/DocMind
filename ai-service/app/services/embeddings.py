"""Embedding generation using Sentence Transformers (all-MiniLM-L6-v2)."""

from __future__ import annotations

import numpy as np
from sentence_transformers import SentenceTransformer


class EmbeddingService:
    """Thin wrapper around SentenceTransformer for batch encoding.

    Loaded once at startup (see app/main.py lifespan) to avoid repeated
    model initialisation overhead.
    """

    def __init__(self, model_name: str = "all-MiniLM-L6-v2") -> None:
        self.model_name = model_name
        # ~85MB – fits within the 1.5GB Python service budget
        self._model = SentenceTransformer(model_name)

    def encode(self, texts: list[str]) -> np.ndarray:
        """Return a 2-D float32 array of shape (len(texts), embedding_dim)."""
        return self._model.encode(texts, convert_to_numpy=True, normalize_embeddings=True)

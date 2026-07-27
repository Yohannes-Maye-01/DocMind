"""POST /embed – generate text embeddings."""

from fastapi import APIRouter, Request
from pydantic import BaseModel

router = APIRouter()


class EmbedRequest(BaseModel):
    texts: list[str]


class EmbedResponse(BaseModel):
    embeddings: list[list[float]]
    model: str


@router.post("", response_model=EmbedResponse)
async def embed(req: EmbedRequest, request: Request) -> EmbedResponse:
    """Generate embeddings for a list of text strings."""
    embedder = request.app.state.embedder
    vectors = embedder.encode(req.texts)
    return EmbedResponse(
        embeddings=vectors.tolist(),
        model=embedder.model_name,
    )

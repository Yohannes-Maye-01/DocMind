"""Runtime configuration loaded from environment variables."""

from pydantic_settings import BaseSettings


class Settings(BaseSettings):
    port: int = 8082
    ollama_url: str = "http://ollama:11434"
    ollama_model: str = "phi:2"
    embedding_model: str = "all-MiniLM-L6-v2"
    # Max chunks to inject into RAG prompt
    rag_top_k: int = 5

    model_config = {"env_file": ".env", "extra": "ignore"}


settings = Settings()

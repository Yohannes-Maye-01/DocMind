DocMind/
├── .github/
├── docs/
├── frontend/
├── backend/
├── search-service/
├── ai-service/
├── shared/
├── infra/
├── scripts/
├── tests/
└── README.md# DocMind AI Project Context

## Project Name
DocMind

## Project Goal
Build an AI-powered document intelligence platform where users can upload documents, index them, search content semantically, and ask questions about their files using retrieval-augmented generation (RAG).

## Current Skill Level
I know the basics of:
- Rust
- Go
- Python
- TypeScript

I am learning by building this project, so the design should remain practical, modular, and beginner-friendly.

## Technology Stack

### Frontend
- TypeScript
- React
- REST/API-friendly UI layer

### Backend
- Go
- REST API
- Service orchestration and business logic

### Search Service
- Rust
- Typesense for fast keyword and vector search
- Designed to be loosely coupled from the main API

### AI Service
- Python
- Embeddings, chunking, RAG workflows, and model integration

### Data Layer
- PostgreSQL for structured data
- Optional object storage for uploaded documents later

### DevOps
- Docker
- Docker Compose
- GitHub Actions
- CI/CD workflow support

## Recommended Repository Structure

A clean monorepo-style structure for this project is:

```text
DocMind/
├── .github/
│   └── workflows/
├── docs/
│   ├── planning/
│   ├── architecture/
│   └── api/
├── frontend/                 # React + TypeScript UI
├── backend/                  # Go REST API service
├── search-service/           # Rust search/indexing service
├── ai-service/               # Python AI/RAG service
├── shared/                   # Shared schemas, contracts, and utilities
│   ├── schemas/
│   └── libs/
├── infra/
│   ├── docker/
│   └── docker-compose.yml
├── scripts/
├── tests/
└── README.md
```

### Suggested service boundaries
- Frontend handles UI and user interaction.
- Backend handles authentication, document management, and API orchestration.
- Search service handles indexing and retrieval.
- AI service handles embeddings, chunking, and RAG logic.
- Shared contains contracts and models used across services.

## SDLC Phase
Current Phase:
Planning / Requirements Analysis

Completed:
- Project idea created
- Core technology choices decided
- Initial repository structure planned

Next Tasks:
1. Define functional requirements
2. Finalize architecture
3. Design database schema
4. Create development roadmap
5. Set up Docker and CI/CD skeleton

## Architecture Idea

```text
User -> Frontend (React/TypeScript)
          |
          v
      Backend API (Go)
          |
    +-----+------------------+
    |     |                  |
    v     v                  v
Search Service (Rust)   AI Service (Python)
    |                        |
    +---------> Typesense <---+
                    |
                    v
                PostgreSQL
```

## Rules for AI Assistants

When helping with this project:
- Explain things step by step
- Follow the SDLC process
- Think like a software architect
- Keep solutions simple and practical
- Consider low-resource development on an 8GB RAM machine
- Prefer free and open-source tools where possible
- Suggest professional engineering practices

## Current Problems / Questions

Add new questions or blockers here as they appear.

## Decisions Made

Add key architecture and implementation decisions here.

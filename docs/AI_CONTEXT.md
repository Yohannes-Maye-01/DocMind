# DocMind AI Project Context

## Project Name
DocMind

## Project Goal
Build an AI-powered document intelligence platform where users can upload documents, search content, and ask questions about their files.

## My Skill Level
I know basics of:
- Rust
- Go
- Python
- TypeScript

I am learning by building this project.

## Technology Stack

Frontend:
- TypeScript
- React

Backend:
- Go
- REST API

Search Service:
- Rust
- Typesense (Search & Indexing Engine; runs via Docker, integrated via typesense-rust or API clients)

AI Service:
- Python

Database:
- PostgreSQL

DevOps:
- Docker
- GitHub Actions
- CI/CD

## SDLC Phase
Current Phase:
Planning / Requirements Analysis

Completed:
- Project idea created
- Technology choices decided

Next Tasks:
1. Define requirements
2. Design architecture
3. Design database
4. Create development roadmap

## Architecture Idea

Frontend (TypeScript)
        |
        |
Go Backend API
        |
   ----------------
   |              |              |
Rust Service   Python AI     Typesense Engine
 (Data Processing &         (Embeddings &   (Lightning-fast keyword & vector
   `typesense-rust`)         RAG Pipelines)   indexing; accessed via HTTP API or client libraries)
        |
        ▼
    PostgreSQL

## Rules for AI Assistants

When helping me:
- Explain step by step
- Follow SDLC
- Think like a software architect
- Do not skip fundamentals
- Consider my 8GB RAM PC and offline development
- Prefer free tools
- Suggest professional practices

## Current Problems / Questions

(Add new questions here)

## Decisions Made

(Add important decisions here)

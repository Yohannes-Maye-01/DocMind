Database Design DocumentDocMind AI1. Database OverviewDatabase Management System: PostgreSQL 16+ (containerized via Docker).Purpose: To store user credentials, document metadata, file storage references, and structural chunk indexes required for fast retrieval and RAG operations.Design Principles: Relational integrity, foreign key constraints with cascading deletes, indexing on frequently queried columns, and optimization for local execution.2. Entity-Relationship (ER) OverviewUsers $\rightarrow$ Documents (One-to-Many: A user can upload many documents).Documents $\rightarrow$ Document Chunks (One-to-Many: A document is split into multiple text chunks for searching and vector embedding).3. Database Schema (SQL DDL)SQL-- Enable UUID extension for unique identifiers
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 1. Users Table
-- Stores authentication credentials and account metadata
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index for fast user lookups during authentication
CREATE INDEX idx_users_email ON users(email);

-- 2. Documents Table
-- Stores metadata regarding uploaded files (PDF, Markdown, Text)
CREATE TABLE documents (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    file_path VARCHAR(512) NOT NULL, -- Local storage path or object reference
    file_size INT NOT NULL,          -- Size in bytes
    file_type VARCHAR(50) NOT NULL,  -- e.g., 'pdf', 'txt', 'md'
    status VARCHAR(50) DEFAULT 'pending', -- 'pending', 'indexed', 'failed'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index for querying all documents belonging to a specific user
CREATE INDEX idx_documents_user_id ON documents(user_id);

-- 3. Document Chunks Table
-- Stores segmented text fragments used by the Rust search engine and Python AI service
CREATE TABLE document_chunks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    document_id UUID NOT NULL REFERENCES documents(id) ON DELETE CASCADE,
    chunk_index INT NOT NULL,        -- Sequential order of the chunk in the document
    content TEXT NOT NULL,           -- The actual text snippet
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Index for retrieving chunks related to a specific document quickly
CREATE INDEX idx_document_chunks_doc_id ON document_chunks(document_id);
4. Data Management & Optimization Strategies (8GB RAM Local Constraints)Connection Pooling: The Go backend will utilize a connection pool (e.g., pgxpool) with a limited max connection count (e.g., 5 to 10 connections) to prevent PostgreSQL from consuming excessive RAM.Storage Footprint: Document binaries are stored on the local disk (mounted via Docker volumes), while database records maintain the file paths (file_path) and metadata, keeping database queries lightweight.Vector Handling: Depending on scale, vector embeddings can either be stored directly in PostgreSQL (using the pgvector extension if needed) or managed via the Rust search engine index. Keeping chunks indexed cleanly ensures swift retrieval during RAG queries.

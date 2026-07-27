# 🧠 DocMind

> An AI-powered document search and knowledge assistant engineered with a high-performance polyglot architecture for lightning-fast retrieval, intelligent RAG processing, and seamless user interaction.

[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Python](https://img.shields.io/badge/Python-3.9%2B-blue)](https://www.python.org/)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![Go](https://img.shields.io/badge/Go-1.21%2B-00ADD8)](https://golang.org/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.0%2B-3178C6)](https://www.typescriptlang.org/)

---

## ⚡ Features

- **Lightning-Fast Retrieval** - Powered by Rust for ultra-optimized performance
- **Intelligent RAG Processing** - Advanced Retrieval-Augmented Generation with Python & AI models
- **Polyglot Architecture** - Seamlessly integrated Rust, Go, Python, and TypeScript microservices
- **Semantic Search** - Find exactly what you need with AI-powered understanding
- **Multi-Format Support** - Process documents in various formats (PDF, DOCX, TXT, JSON, etc.)
- **Real-Time Indexing** - Instant document processing and indexing
- **Scalable Design** - Built for enterprise-grade performance and reliability

---

## 🏗️ Architecture

DocMind leverages a modern polyglot microservices architecture:

```
┌─────────────────────────────────────────────────────────┐
│                    Frontend (TypeScript)                │
└──────────────────────┬──────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────┐
│          API Gateway & Services (Go)                    │
└──────────────────────┬──────────────────────────────────┘
                       │
        ┌──────────────┼──────────────┐
        │              │              │
        ▼              ▼              ▼
   ┌────────┐    ┌─────────┐    ┌─────────┐
   │  Rust  │    │ Python  │    │   Go    │
   │ Engine │    │  RAG    │    │ Workers │
   └────────┘    └─────────┘    └─────────┘
```

---

## 🚀 Quick Start

### Prerequisites
- Node.js 18+
- Python 3.9+
- Rust 1.70+
- Go 1.21+
- Docker & Docker Compose (recommended)

### Installation

```bash
# Clone the repository
git clone https://github.com/Yohannes-Maye-01/DocMind.git
cd DocMind

# Using Docker (recommended)
docker-compose up -d

# Or install dependencies manually
npm install
pip install -r requirements.txt
cargo build --release
go mod download
```

### Usage

```bash
# Start the application
npm start

# Run in development mode
npm run dev

# Run tests
npm test
```

---

## 📦 Project Structure

```
DocMind/
├── .github/                # GitHub workflows and automation
├── docs/                   # Project documentation
│   ├── planning/           # Planning and requirements docs
│   ├── architecture/       # Architecture notes
│   └── api/                # API documentation
├── frontend/               # React + TypeScript UI
├── backend/                # Go REST API service
├── search-service/         # Rust search and indexing service
├── ai-service/             # Python AI/RAG processing service
├── shared/                 # Shared schemas and libraries
│   ├── schemas/            # Shared request/response models
│   └── libs/               # Shared utility code
├── infra/                  # Infrastructure and deployment files
│   └── docker/             # Docker-related assets
├── scripts/                # Utility scripts
├── tests/                  # Test suites
├── README.md               # Project overview
└── infra/docker-compose.yml
```

---

## 🔗 API Endpoints

- `POST /api/documents/upload` - Upload documents
- `GET /api/search?q=query` - Semantic search
- `POST /api/rag/query` - RAG-powered queries
- `GET /api/documents/:id` - Retrieve document
- `DELETE /api/documents/:id` - Delete document

---

## 📚 Documentation

For detailed documentation, check out:
- [API Documentation](./docs/api.md)
- [Architecture Guide](./docs/architecture.md)
- [Setup Guide](./docs/setup.md)
- [Contributing Guidelines](./CONTRIBUTING.md)

---

## 🛠️ Tech Stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Search Engine | Rust | High-performance indexing & retrieval |
| AI/ML Processing | Python | RAG, embeddings, NLP |
| API Services | Go | High-concurrency request handling |
| Frontend | TypeScript/React | User interface |
| Database | PostgreSQL | Persistent storage |
| Cache | Redis | Fast caching layer |

---

## 🤝 Contributing

We welcome contributions! Please read our [CONTRIBUTING.md](./CONTRIBUTING.md) file for details on our code of conduct and the process for submitting pull requests.

```bash
# Create a feature branch
git checkout -b feature/amazing-feature

# Commit changes
git commit -m 'Add amazing feature'

# Push to branch
git push origin feature/amazing-feature

# Open a Pull Request
```

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 💡 Roadmap

- [ ] Multi-language support
- [ ] Advanced NLP models
- [ ] Collaborative features
- [ ] Mobile app
- [ ] Enterprise deployment options
- [ ] Custom model training
- [ ] API rate limiting & analytics

---

## 📞 Support & Contact

- **Issues & Bugs**: [GitHub Issues](https://github.com/Yohannes-Maye-01/DocMind/issues)
- **Discussions**: [GitHub Discussions](https://github.com/Yohannes-Maye-01/DocMind/discussions)
- **Email**: support@docmind.dev

---

## ⭐ Show Your Support

If you find DocMind helpful, please give us a star! It helps us grow and improve.

---

**Made with ❤️ by the DocMind Team**

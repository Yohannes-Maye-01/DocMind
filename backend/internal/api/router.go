package api

import (
	"net/http"

	"github.com/go-chi/chi/v5"
	chimiddleware "github.com/go-chi/chi/v5/middleware"
	"github.com/go-chi/cors"
	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/docmind/backend/internal/api/handlers"
	"github.com/docmind/backend/internal/api/middleware"
	"github.com/docmind/backend/internal/config"
)

// NewRouter builds and returns the main HTTP router.
func NewRouter(cfg *config.Config, db *pgxpool.Pool) http.Handler {
	r := chi.NewRouter()

	// ── Global middleware ──────────────────────────────────
	r.Use(chimiddleware.RequestID)
	r.Use(chimiddleware.RealIP)
	r.Use(middleware.StructuredLogger)
	r.Use(chimiddleware.Recoverer)
	r.Use(chimiddleware.Timeout(30 * 1e9)) // 30s request timeout

	// CORS – restrict to frontend origin in production
	r.Use(cors.Handler(cors.Options{
		AllowedOrigins:   []string{"http://localhost:3000"},
		AllowedMethods:   []string{"GET", "POST", "PUT", "DELETE", "OPTIONS"},
		AllowedHeaders:   []string{"Authorization", "Content-Type"},
		AllowCredentials: true,
		MaxAge:           300,
	}))

	// ── Health endpoints ───────────────────────────────────
	h := handlers.NewHealthHandler(db, cfg)
	r.Get("/health", h.Overall)
	r.Get("/health/database", h.Database)
	r.Get("/health/services", h.Services)

	// ── API v1 ─────────────────────────────────────────────
	r.Route("/api", func(r chi.Router) {
		// Documents
		r.Route("/documents", func(r chi.Router) {
			r.Get("/", handlers.ListDocuments(db))
			r.Post("/upload", handlers.UploadDocument(cfg, db))
			r.Get("/{id}", handlers.GetDocument(db))
			r.Delete("/{id}", handlers.DeleteDocument(cfg, db))
		})

		// Search
		r.Get("/search", handlers.Search(cfg))

		// Q&A / RAG
		r.Post("/query", handlers.RAGQuery(cfg))
	})

	return r
}

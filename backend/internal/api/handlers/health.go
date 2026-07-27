package handlers

import (
	"context"
	"encoding/json"
	"net/http"
	"time"

	"github.com/jackc/pgx/v5/pgxpool"

	"github.com/docmind/backend/internal/config"
)

type HealthHandler struct {
	db  *pgxpool.Pool
	cfg *config.Config
}

func NewHealthHandler(db *pgxpool.Pool, cfg *config.Config) *HealthHandler {
	return &HealthHandler{db: db, cfg: cfg}
}

type healthResponse struct {
	Status    string            `json:"status"`
	Timestamp string            `json:"timestamp"`
	Services  map[string]string `json:"services,omitempty"`
}

// Overall returns overall system health.
func (h *HealthHandler) Overall(w http.ResponseWriter, r *http.Request) {
	respondJSON(w, http.StatusOK, healthResponse{
		Status:    "ok",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
	})
}

// Database checks PostgreSQL connectivity.
func (h *HealthHandler) Database(w http.ResponseWriter, r *http.Request) {
	ctx, cancel := context.WithTimeout(r.Context(), 3*time.Second)
	defer cancel()

	if err := h.db.Ping(ctx); err != nil {
		respondJSON(w, http.StatusServiceUnavailable, healthResponse{
			Status:    "error",
			Timestamp: time.Now().UTC().Format(time.RFC3339),
		})
		return
	}

	respondJSON(w, http.StatusOK, healthResponse{
		Status:    "ok",
		Timestamp: time.Now().UTC().Format(time.RFC3339),
	})
}

// Services checks downstream Rust and Python service reachability.
func (h *HealthHandler) Services(w http.ResponseWriter, r *http.Request) {
	statuses := map[string]string{
		"search-service": checkUpstream(h.cfg.RustServiceURL + "/health"),
		"ai-service":     checkUpstream(h.cfg.PythonServiceURL + "/health"),
	}

	overall := "ok"
	for _, s := range statuses {
		if s != "ok" {
			overall = "degraded"
			break
		}
	}

	code := http.StatusOK
	if overall == "degraded" {
		code = http.StatusServiceUnavailable
	}

	respondJSON(w, code, healthResponse{
		Status:    overall,
		Timestamp: time.Now().UTC().Format(time.RFC3339),
		Services:  statuses,
	})
}

func checkUpstream(url string) string {
	client := &http.Client{Timeout: 3 * time.Second}
	resp, err := client.Get(url)
	if err != nil || resp.StatusCode >= 400 {
		return "unreachable"
	}
	return "ok"
}

// ── Stub document & search handlers (implemented in later phases) ─────────────

func ListDocuments(db *pgxpool.Pool) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		respondJSON(w, http.StatusOK, map[string]any{"documents": []any{}, "total": 0})
	}
}

func UploadDocument(cfg *config.Config, db *pgxpool.Pool) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		respondJSON(w, http.StatusNotImplemented, map[string]string{"message": "not yet implemented"})
	}
}

func GetDocument(db *pgxpool.Pool) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		respondJSON(w, http.StatusNotImplemented, map[string]string{"message": "not yet implemented"})
	}
}

func DeleteDocument(cfg *config.Config, db *pgxpool.Pool) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		respondJSON(w, http.StatusNotImplemented, map[string]string{"message": "not yet implemented"})
	}
}

func Search(cfg *config.Config) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		respondJSON(w, http.StatusNotImplemented, map[string]string{"message": "not yet implemented"})
	}
}

func RAGQuery(cfg *config.Config) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		respondJSON(w, http.StatusNotImplemented, map[string]string{"message": "not yet implemented"})
	}
}

// ── Helpers ───────────────────────────────────────────────────────────────────

func respondJSON(w http.ResponseWriter, status int, body any) {
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(status)
	_ = json.NewEncoder(w).Encode(body)
}

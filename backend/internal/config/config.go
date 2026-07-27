package config

import (
	"os"
)

// Config holds all runtime configuration loaded from environment variables.
type Config struct {
	Port               string
	PostgresDSN        string
	JWTSecret          string
	RustServiceURL     string
	PythonServiceURL   string
	TypesenseURL       string
	TypesenseAPIKey    string
	TypesenseCollection string
}

// Load reads configuration from environment variables.
// It panics on missing required values so misconfigured deployments fail fast.
func Load() *Config {
	return &Config{
		Port:                getEnv("PORT", "8080"),
		PostgresDSN:         requireEnv("POSTGRES_DSN"),
		JWTSecret:           requireEnv("JWT_SECRET"),
		RustServiceURL:      getEnv("RUST_SERVICE_URL", "http://localhost:8081"),
		PythonServiceURL:    getEnv("PYTHON_SERVICE_URL", "http://localhost:8082"),
		TypesenseURL:        getEnv("TYPESENSE_URL", "http://localhost:8108"),
		TypesenseAPIKey:     requireEnv("TYPESENSE_API_KEY"),
		TypesenseCollection: getEnv("TYPESENSE_COLLECTION", "docmind_chunks"),
	}
}

func getEnv(key, defaultVal string) string {
	if v := os.Getenv(key); v != "" {
		return v
	}
	return defaultVal
}

func requireEnv(key string) string {
	v := os.Getenv(key)
	if v == "" {
		panic("required environment variable not set: " + key)
	}
	return v
}

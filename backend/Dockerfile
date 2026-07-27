# Multi-stage build for Go backend
# Stage 1: Build
FROM golang:1.21-alpine as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
    git \
    ca-certificates

# Copy go mod files
COPY backend/go.mod backend/go.sum ./

# Download dependencies
RUN go mod download

# Copy source code
COPY backend/ .

# Build application
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -a -installsuffix cgo -o backend .

# Stage 2: Runtime
FROM alpine:3.18

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    curl \
    postgresql-client

# Create non-root user for security
RUN addgroup -g 1001 appuser && \
    adduser -D -u 1001 -G appuser appuser && \
    chown -R appuser:appuser /app

# Copy binary from builder
COPY --from=builder /app/backend /app/backend

USER appuser

# Expose port
EXPOSE 8000

# Environment variables
ENV GO_ENV=production

# Health check
HEALTHCHECK --interval=10s --timeout=5s --retries=5 \
    CMD curl -f http://localhost:8000/health || exit 1

# Run application
CMD ["/app/backend"]

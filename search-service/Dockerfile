# Multi-stage build for Rust search service
# Stage 1: Build
FROM rust:1.70-alpine as builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkg-config \
    openssl-dev

# Copy Cargo files
COPY search-service/service/Cargo.toml Cargo.toml
COPY search-service/service/Cargo.lock Cargo.lock

# Create dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy actual source
COPY search-service/service/src src

# Build application
RUN cargo build --release

# Stage 2: Runtime
FROM alpine:3.18

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libssl3 \
    curl

# Copy binary from builder
COPY --from=builder /app/target/release/search-service /app/search-service

# Create non-root user for security
RUN addgroup -g 1001 appuser && \
    adduser -D -u 1001 -G appuser appuser && \
    chown -R appuser:appuser /app

USER appuser

# Expose port
EXPOSE 8001

# Health check
HEALTHCHECK --interval=10s --timeout=5s --retries=5 \
    CMD curl -f http://localhost:8001/health || exit 1

# Run application
CMD ["/app/search-service"]

# ── Stage 1: Build ──────────────────────────────
FROM rust:1.79-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml ./
# Pre-fetch dependencies (layer cache)
RUN mkdir src && echo "fn main(){}" > src/main.rs && cargo build --release 2>/dev/null; rm -rf src

COPY src ./src
RUN cargo build --release

# ── Stage 2: Runtime ────────────────────────────
FROM alpine:3.20

RUN addgroup -S docmind && adduser -S docmind -G docmind
WORKDIR /app
COPY --from=builder /app/target/release/search-service /app/search-service
RUN chown docmind:docmind /app/search-service
USER docmind

EXPOSE 8081
HEALTHCHECK --interval=15s --timeout=5s --retries=3 \
  CMD wget -qO- http://localhost:8081/health || exit 1

ENTRYPOINT ["/app/search-service"]

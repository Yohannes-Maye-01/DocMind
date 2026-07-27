# ── Stage 1: Dependencies ───────────────────────
FROM python:3.11-slim AS deps

WORKDIR /app
COPY requirements.txt .
RUN pip install --no-cache-dir --prefix=/install -r requirements.txt

# ── Stage 2: Runtime ────────────────────────────
FROM python:3.11-slim

RUN addgroup --system docmind && adduser --system --ingroup docmind docmind
WORKDIR /app

COPY --from=deps /install /usr/local
COPY app/ ./app/

RUN chown -R docmind:docmind /app
USER docmind

EXPOSE 8082
HEALTHCHECK --interval=20s --timeout=10s --retries=3 \
  CMD curl -f http://localhost:8082/health || exit 1

CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8082"]

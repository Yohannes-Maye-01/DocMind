#!/usr/bin/env bash
# DocMind – One-command local deployment
# Usage: ./scripts/deploy.sh

set -euo pipefail

echo "==> DocMind Deploy"

# Ensure .env exists
if [ ! -f .env ]; then
  echo "ERROR: .env not found. Run ./scripts/setup.sh first."
  exit 1
fi

# Build and start all services
docker compose down --remove-orphans
docker compose build --parallel
docker compose up -d

echo ""
echo "==> Waiting for services to be healthy..."
sleep 5

# Check health
for svc in backend search-service ai-service; do
  status=$(docker compose ps --format json "$svc" 2>/dev/null | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('Health','unknown'))" 2>/dev/null || echo "unknown")
  echo "  $svc: $status"
done

echo ""
echo "==> DocMind is running:"
echo "   Frontend  → http://localhost:3000"
echo "   Backend   → http://localhost:8080"
echo "   Search    → http://localhost:8081"
echo "   AI        → http://localhost:8082"

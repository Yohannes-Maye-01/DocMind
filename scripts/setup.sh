#!/usr/bin/env bash
# DocMind – Local setup script
# Run once after cloning: ./scripts/setup.sh

set -euo pipefail

echo "==> DocMind Setup"

# 1. Create .env if missing
if [ ! -f .env ]; then
  cp .env.example .env
  echo "  ✓ Created .env from template — fill in real values before starting."
else
  echo "  ✓ .env already exists."
fi

# 2. Check required tools
for cmd in docker docker-compose go rustc python3; do
  if command -v "$cmd" &>/dev/null; then
    echo "  ✓ $cmd found"
  else
    echo "  ✗ $cmd not found — please install it."
  fi
done

echo ""
echo "==> Done. Next: edit .env, then run: make up-build"

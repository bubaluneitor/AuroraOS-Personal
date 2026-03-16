#!/usr/bin/env bash
set -euo pipefail
source .venv/bin/activate || true

# Start Redis in background if not running
if ! nc -z localhost 6379; then
  echo "Starting redis-server..."
  redis-server --daemonize yes
fi

# Run backend
echo "Starting backend..."
uvicorn backend.main:app --host 0.0.0.0 --port 8000 --reload &
sleep 1

# Run scheduler in background
python3 -u backend/scheduler.py &
sleep 1

echo "All services started. Frontend: cd hub/frontend && npm run dev"

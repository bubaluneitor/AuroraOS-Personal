#!/usr/bin/env bash
set -euo pipefail
echo "Installing system dependencies (Debian/Ubuntu)..."

sudo apt update
sudo apt install -y python3 python3-venv python3-pip git curl nodejs npm redis-server unzip netcat-openbsd

echo "Creating Python virtual environment in .venv..."
python3 -m venv .venv
source .venv/bin/activate

pip install --upgrade pip
pip install -r backend/requirements.txt || true

echo "Installation finished. See README.md for usage."

# AuroraOS — Guía definitiva (v0.1)

## Objetivo
Guía paso a paso para instalar y usar AuroraOS Personal.

## Requisitos
- Ubuntu 22.04 (VPS) o similar
- Python 3.10+
- Node 16+
- Redis (recomendado)

## 1 — Clonar repositorio
git clone <TU_REPO_URL> auroraos-personal
cd auroraos-personal

## 2 — Instalar
chmod +x deploy/install.sh
./deploy/install.sh

## 3 — Iniciar todo (local)
./deploy/run_local.sh

## 4 — Crear agentes (ejemplo)
curl -X POST "http://VPS:8000/agents" -H "Content-Type: application/json" -d '{"name":"research-1","kind":"research"}'
curl -X POST "http://VPS:8000/tasks" -H "Content-Type: application/json" -d '{"type":"research","payload":{"topic":"AI research"}}'

## 5 — Conectar RunPod
- Crea un worker que haga polling de /tasks o que reciba tareas vía webhook.

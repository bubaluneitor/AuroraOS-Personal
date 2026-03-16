# Arquitectura AuroraOS (resumen)

- Core API: FastAPI
- Agent Manager: Python asyncio
- Scheduler: dispatcher basado en asyncio.Queue
- Knowledge: (pendiente) PostgreSQL + vector DB
- Hub UI: Next.js
- Workers GPU: RunPod / Docker
- Identity: claves + embeddings (opcional)

# AuroraOS Personal

Versión inicial (v0.1) — Sistema personal de agentes IA para control desde móvil.

## Qué contiene
- Backend (FastAPI) para gestión de agentes/tareas.
- Scheduler asincrónico.
- Agentes ejemplo: research, code, data.
- Hub web (Next.js) con interfaz para controlar el sistema.
- Scripts de despliegue local y release.

## Instalación rápida (local)
```bash
./deploy/install.sh
./deploy/run_local.sh
```

## Cómo usar
1. Inicia backend: `uvicorn backend.main:app --host 0.0.0.0 --port 8000`
2. Inicia frontend: `cd hub/frontend && npm run dev`
3. Abre `http://localhost:3000` en tu teléfono o navegador.

## Licencia
MIT

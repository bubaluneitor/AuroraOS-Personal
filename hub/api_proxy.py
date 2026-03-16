from fastapi import FastAPI
import httpx
import os

app = FastAPI()
BACKEND = os.environ.get("AURORA_BACKEND", "http://localhost:8000")

@app.get("/api/proxy/health")
async def proxy_health():
    async with httpx.AsyncClient() as client:
        r = await client.get(f"{BACKEND}/health")
        return r.json()

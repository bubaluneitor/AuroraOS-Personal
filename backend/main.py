from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
import uuid
from backend.agent_manager import AgentManager
from backend.utils import now_iso

app = FastAPI(title="Aurora Core API", version="0.1.0")
manager = AgentManager()

class AgentCreate(BaseModel):
    name: str
    kind: str = "generic"

class TaskCreate(BaseModel):
    type: str
    payload: dict = {}

@app.get("/health")
async def health():
    return {"status": "ok", "time": now_iso()}

@app.post("/agents", status_code=201)
async def create_agent(req: AgentCreate):
    agent = manager.create_agent(req.name, req.kind)
    return agent.to_dict()

@app.get("/agents")
async def list_agents():
    return [a.to_dict() for a in manager.list_agents()]

@app.post("/tasks", status_code=201)
async def create_task(req: TaskCreate):
    task_id = manager.submit_task(req.type, req.payload)
    return {"task_id": task_id}

@app.get("/tasks")
async def list_tasks():
    return manager.list_tasks()

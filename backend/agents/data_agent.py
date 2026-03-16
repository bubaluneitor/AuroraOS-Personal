import asyncio
from .base_agent import BaseAgent
class DataAgent(BaseAgent):
    def __init__(self, name: str):
        super().__init__(name, kind="data")

    async def run(self, task: dict):
        source = task.get("payload", {}).get("source", "web")
        await asyncio.sleep(0.1)
        return {"status": "done", "result": f"data collected from {source}"}

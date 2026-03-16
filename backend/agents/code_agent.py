import asyncio
from .base_agent import BaseAgent
class CodeAgent(BaseAgent):
    def __init__(self, name: str):
        super().__init__(name, kind="code")

    async def run(self, task: dict):
        spec = task.get("payload", {}).get("spec", "task")
        await asyncio.sleep(0.1)
        return {"status": "done", "result": f"code generated for {spec}"}

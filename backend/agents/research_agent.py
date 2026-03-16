import asyncio
from .base_agent import BaseAgent
class ResearchAgent(BaseAgent):
    def __init__(self, name: str):
        super().__init__(name, kind="research")

    async def run(self, task: dict):
        topic = task.get("payload", {}).get("topic", "general")
        await asyncio.sleep(0.1)
        result = {"summary": f"research summary for {topic}", "source": "venice"}
        return {"status": "done", "result": result}

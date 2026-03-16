import asyncio
class BaseAgent:
    def __init__(self, name: str, kind: str = "generic"):
        self.name = name
        self.kind = kind

    async def run(self, task: dict):
        await asyncio.sleep(0.05)
        return {"status": "ok", "task": task, "agent": self.name}

import asyncio
import uuid
from typing import Dict, List
from backend.agents.base_agent import BaseAgent
from backend.agents.research_agent import ResearchAgent
from backend.agents.code_agent import CodeAgent
from backend.agents.data_agent import DataAgent
from backend.utils import now_iso

class AgentRecord:
    def __init__(self, agent: BaseAgent):
        self.id = str(uuid.uuid4())
        self.agent = agent
        self.status = "idle"

    def to_dict(self):
        return {"id": self.id, "name": self.agent.name, "status": self.status, "kind": self.agent.kind}

class AgentManager:
    def __init__(self):
        self._agents: Dict[str, AgentRecord] = {}
        self._task_queue: asyncio.Queue = asyncio.Queue()
        self._lock = asyncio.Lock()
        # start background loop
        try:
            loop = asyncio.get_event_loop()
            loop.create_task(self._dispatcher_loop())
        except RuntimeError:
            # started from a thread without a loop; create a new loop in a thread
            import threading
            threading.Thread(target=self._start_loop_thread, daemon=True).start()

    def _start_loop_thread(self):
        loop = asyncio.new_event_loop()
        asyncio.set_event_loop(loop)
        loop.create_task(self._dispatcher_loop())
        loop.run_forever()

    def create_agent(self, name: str, kind: str="generic") -> AgentRecord:
        if kind == "research":
            agent = ResearchAgent(name)
        elif kind == "code":
            agent = CodeAgent(name)
        elif kind == "data":
            agent = DataAgent(name)
        else:
            agent = BaseAgent(name)
        rec = AgentRecord(agent)
        self._agents[rec.id] = rec
        return rec

    def list_agents(self) -> List[AgentRecord]:
        return list(self._agents.values())

    def submit_task(self, ttype: str, payload: dict) -> str:
        task_id = str(uuid.uuid4())
        task = {"id": task_id, "type": ttype, "payload": payload, "created": now_iso()}
        # enqueue safely from any thread
        asyncio.get_event_loop().call_soon_threadsafe(self._task_queue.put_nowait, task)
        return task_id

    def list_tasks(self) -> List[dict]:
        try:
            items = list(self._task_queue._queue)
            return items
        except Exception:
            return []

    async def _dispatcher_loop(self):
        while True:
            task = await self._task_queue.get()
            assigned = False
            # simple round-robin: find an idle agent
            for rec in self._agents.values():
                if rec.status == "idle":
                    rec.status = "busy"
                    asyncio.create_task(self._run_task(rec, task))
                    assigned = True
                    break
            if not assigned:
                # if no idle agent, requeue with small delay
                await asyncio.sleep(0.2)
                await self._task_queue.put(task)

    async def _run_task(self, rec: AgentRecord, task: dict):
        try:
            result = await rec.agent.run(task)
            rec.status = "idle"
            # TODO: persist result in knowledge store
        except Exception as e:
            rec.status = "idle"
            # log error

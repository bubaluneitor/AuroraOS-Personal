import asyncio
from backend.utils import now_iso

async def heartbeat():
    while True:
        print(f"[scheduler] heartbeat {now_iso()}")
        await asyncio.sleep(10)

if __name__ == "__main__":
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.create_task(heartbeat())
    loop.run_forever()

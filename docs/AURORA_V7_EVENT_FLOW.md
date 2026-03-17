# Aurora v7.0 — Flujo de datos y eventos

```text
                    +----------------------------+
                    |        Control Hub         |
                    | (UI tiempo real, alertas)  |
                    +-------------^--------------+
                                  |
                                  | subscribe
                                  |
+--------------------+     +------+-------+      +---------------------------+
| Agent & Model      |     |  Event Bus   |      | Multimodal Memory System |
| Manager            +----->  (NATS/RMQ)  +----->| - Database               |
| - registro plugins | evt |              | evt  | - Vector Memory          |
| - catálogo modelos |     +------+-------+      | - Asset Library          |
+---------+----------+            |              | - Knowledge Graph        |
          |                       |              +---------------------------+
          | select agent/model    |
          v                       |
+---------------------------+     |
| Intelligent Task Router   |-----+
| - scoring coste/beneficio |
| - policy seguridad        |
+------------+--------------+
             |
             | dispatch
             v
   +---------+---------+---------+----------------+
   |                   |                          |
+--+---------+  +------+-------+          +-------+-------+
| Security   |  | MediaAgent   |          | Automation   |
| Agent      |  | (Kling/Hailuo|          | Agent        |
| (OSINT/CWE)|  | /Seedance)   |          | (Emergent/   |
+------+-----+  +------+-------+          | Freepik)     |
       |               |                  +-------+------+
       +---------------+--------------------------+
                       |
                       | task_completed / artifacts_created
                       v
                +------+------------------+
                | Asset Library + Reports |
                +-------------------------+
```

## Ejemplo de enrutamiento
- Tarea de lógica (`TaskType::Reasoning`) → `SecurityAgent` + `deepseek`.
- Tarea de generación de imagen (`TaskType::ImageGeneration`) → `MediaAgent` + `qwen-3.5-vision`.

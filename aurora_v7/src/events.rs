use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SystemEvent {
    TaskReceived {
        task_id: String,
        task_type: String,
    },
    RouteComputed {
        task_id: String,
        agent: String,
        model: String,
    },
    AgentFailed {
        agent: String,
        failure_count: u32,
    },
    AgentHealed {
        agent: String,
        strategy: String,
    },
    SecurityAlert {
        task_id: String,
        severity: String,
        details: String,
    },
    TaskCompleted {
        task_id: String,
        output_ref: String,
    },
}

pub trait EventPublisher: Send + Sync {
    fn publish(&mut self, event: SystemEvent);
}

#[derive(Default)]
pub struct InMemoryEventBus {
    pub events: Vec<SystemEvent>,
}

impl EventPublisher for InMemoryEventBus {
    fn publish(&mut self, event: SystemEvent) {
        self.events.push(event);
    }
}

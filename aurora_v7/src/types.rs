use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TaskType {
    Reasoning,
    ImageGeneration,
    VideoGeneration,
    Automation,
    SecurityAudit,
    Research,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub task_type: TaskType,
}

impl Task {
    pub fn new(description: impl Into<String>, task_type: TaskType) -> Self {
        Self {
            id: Uuid::new_v4(),
            description: description.into(),
            task_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: Uuid,
    pub output: String,
    pub artifacts: Vec<String>,
}

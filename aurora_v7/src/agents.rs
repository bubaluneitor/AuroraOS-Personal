use crate::models::ModelCapability;
use crate::types::{Task, TaskResult, TaskType};

pub trait Agent: Send + Sync {
    fn id(&self) -> &str;
    fn capabilities(&self) -> &[TaskType];
    fn health_check(&self) -> bool;
    fn execute(&self, task: &Task, model_response: &str) -> TaskResult;
}

pub struct MediaAgent {
    id: String,
    capabilities: Vec<TaskType>,
}

impl MediaAgent {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            capabilities: vec![TaskType::ImageGeneration, TaskType::VideoGeneration],
        }
    }

    pub fn build_prompt(&self, task: &Task) -> String {
        format!(
            "[MediaAgent:{}] Crea prompt multimodal avanzado para Kling 3.0 / Hailuo / Seedance: {}",
            self.id, task.description
        )
    }
}

impl Agent for MediaAgent {
    fn id(&self) -> &str {
        &self.id
    }

    fn capabilities(&self) -> &[TaskType] {
        &self.capabilities
    }

    fn health_check(&self) -> bool {
        true
    }

    fn execute(&self, task: &Task, model_response: &str) -> TaskResult {
        TaskResult {
            task_id: task.id,
            output: format!("Media plan generado: {model_response}"),
            artifacts: vec!["asset://storyboard/v1".to_string()],
        }
    }
}

pub struct AutomationAgent {
    id: String,
    capabilities: Vec<TaskType>,
}

impl AutomationAgent {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            capabilities: vec![TaskType::Automation],
        }
    }

    pub fn build_prompt(&self, task: &Task) -> String {
        format!(
            "[AutomationAgent:{}] Traduce objetivo de negocio a flujos Emergent/Freepik Spaces: {}",
            self.id, task.description
        )
    }
}

impl Agent for AutomationAgent {
    fn id(&self) -> &str {
        &self.id
    }

    fn capabilities(&self) -> &[TaskType] {
        &self.capabilities
    }

    fn health_check(&self) -> bool {
        true
    }

    fn execute(&self, task: &Task, model_response: &str) -> TaskResult {
        TaskResult {
            task_id: task.id,
            output: format!("Blueprint no-code listo: {model_response}"),
            artifacts: vec!["asset://automation/spec/v1".to_string()],
        }
    }
}

pub struct SecurityAgent {
    id: String,
    capabilities: Vec<TaskType>,
}

impl SecurityAgent {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            capabilities: vec![TaskType::SecurityAudit, TaskType::Reasoning],
        }
    }

    pub fn build_prompt(&self, task: &Task) -> String {
        format!(
            "[SecurityAgent:{}] Ejecuta auditoría OSINT + revisión CWE/OWASP: {}",
            self.id, task.description
        )
    }
}

impl Agent for SecurityAgent {
    fn id(&self) -> &str {
        &self.id
    }

    fn capabilities(&self) -> &[TaskType] {
        &self.capabilities
    }

    fn health_check(&self) -> bool {
        true
    }

    fn execute(&self, task: &Task, model_response: &str) -> TaskResult {
        TaskResult {
            task_id: task.id,
            output: format!("Security findings: {model_response}"),
            artifacts: vec!["asset://security/report/v1".to_string()],
        }
    }
}

pub fn task_model_hint(task_type: &TaskType) -> ModelCapability {
    match task_type {
        TaskType::Reasoning | TaskType::SecurityAudit | TaskType::Research => {
            ModelCapability::Reasoning
        }
        TaskType::ImageGeneration => ModelCapability::ImageGeneration,
        TaskType::VideoGeneration => ModelCapability::VideoGeneration,
        TaskType::Automation => ModelCapability::Coding,
    }
}

use crate::types::TaskType;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ModelCapability {
    Reasoning,
    Vision,
    Audio,
    Coding,
    ImageGeneration,
    VideoGeneration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub name: String,
    pub capabilities: Vec<ModelCapability>,
    pub cost_per_1k_tokens: f32,
    pub efficiency_score: f32,
}

pub trait ModelProvider: Send + Sync {
    fn invoke(&self, model: &str, prompt: &str) -> Result<String, String>;
    fn list_models(&self) -> Vec<ModelMetadata>;
}

#[derive(Debug, Clone)]
pub struct StaticModelProvider {
    models: Vec<ModelMetadata>,
}

impl StaticModelProvider {
    pub fn new(models: Vec<ModelMetadata>) -> Self {
        Self { models }
    }

    pub fn required_capability(task_type: &TaskType) -> ModelCapability {
        match task_type {
            TaskType::Reasoning | TaskType::SecurityAudit | TaskType::Research => {
                ModelCapability::Reasoning
            }
            TaskType::ImageGeneration => ModelCapability::ImageGeneration,
            TaskType::VideoGeneration => ModelCapability::VideoGeneration,
            TaskType::Automation => ModelCapability::Coding,
        }
    }
}

impl ModelProvider for StaticModelProvider {
    fn invoke(&self, model: &str, prompt: &str) -> Result<String, String> {
        Ok(format!("[{} simulated response] {}", model, prompt))
    }

    fn list_models(&self) -> Vec<ModelMetadata> {
        self.models.clone()
    }
}

use std::collections::HashMap;

use crate::agents::Agent;
use crate::models::{ModelCapability, ModelMetadata};

pub struct AgentAndModelManager {
    agents: HashMap<String, Box<dyn Agent>>,
    models: Vec<ModelMetadata>,
    failures: HashMap<String, u32>,
}

impl AgentAndModelManager {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            models: Vec::new(),
            failures: HashMap::new(),
        }
    }

    pub fn register_agent(&mut self, agent: Box<dyn Agent>) {
        self.agents.insert(agent.id().to_string(), agent);
    }

    pub fn register_model(
        &mut self,
        name: impl Into<String>,
        capabilities: Vec<ModelCapability>,
        cost_per_1k_tokens: f32,
        efficiency_score: f32,
    ) {
        self.models.push(ModelMetadata {
            name: name.into(),
            capabilities,
            cost_per_1k_tokens,
            efficiency_score,
        });
    }

    pub fn compatible_agents(&self, task_type: &crate::types::TaskType) -> Vec<&dyn Agent> {
        self.agents
            .values()
            .filter(|a| a.capabilities().contains(task_type))
            .map(|a| a.as_ref())
            .collect()
    }

    pub fn models(&self) -> &Vec<ModelMetadata> {
        &self.models
    }

    pub fn register_failure(&mut self, agent_id: &str) -> u32 {
        let entry = self.failures.entry(agent_id.to_string()).or_insert(0);
        *entry += 1;
        *entry
    }

    pub fn auto_heal_action(&self, agent_id: &str) -> &'static str {
        match self.failures.get(agent_id).copied().unwrap_or_default() {
            0..=2 => "restart",
            3..=4 => "disable_temporarily",
            _ => "escalate_to_control_hub",
        }
    }
}

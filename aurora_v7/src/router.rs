use thiserror::Error;

use crate::manager::AgentAndModelManager;
use crate::memory::MemoryStore;
use crate::models::{ModelProvider, StaticModelProvider};
use crate::types::Task;

#[derive(Debug, Clone)]
pub struct RouteDecision {
    pub selected_agent: String,
    pub selected_model: String,
    pub score: f32,
}

#[derive(Debug, Error)]
pub enum RouterError {
    #[error("no compatible agent found")]
    NoAgent,
    #[error("no compatible model found")]
    NoModel,
}

pub struct IntelligentTaskRouter<P: ModelProvider> {
    manager: AgentAndModelManager,
    provider: P,
}

impl<P: ModelProvider> IntelligentTaskRouter<P> {
    pub fn new(manager: AgentAndModelManager, provider: P) -> Self {
        Self { manager, provider }
    }

    pub fn route(
        &self,
        task: &Task,
        memory: &mut dyn MemoryStore,
    ) -> Result<RouteDecision, RouterError> {
        let compatible_agents = self.manager.compatible_agents(&task.task_type);
        let agent = compatible_agents.first().ok_or(RouterError::NoAgent)?;

        let required = StaticModelProvider::required_capability(&task.task_type);
        let selected_model = self
            .provider
            .list_models()
            .into_iter()
            .filter(|m| m.capabilities.contains(&required))
            .max_by(|a, b| {
                score_model(a.efficiency_score, a.cost_per_1k_tokens)
                    .total_cmp(&score_model(b.efficiency_score, b.cost_per_1k_tokens))
            })
            .ok_or(RouterError::NoModel)?;

        let score = score_model(
            selected_model.efficiency_score,
            selected_model.cost_per_1k_tokens,
        );

        memory.save(
            "routes",
            &task.id.to_string(),
            &format!("{}|{}", agent.id(), selected_model.name),
        );

        Ok(RouteDecision {
            selected_agent: agent.id().to_string(),
            selected_model: selected_model.name,
            score,
        })
    }
}

fn score_model(efficiency: f32, cost_per_1k_tokens: f32) -> f32 {
    // Algoritmo coste-beneficio-eficiencia:
    // mayor eficiencia y menor coste maximizan la puntuación.
    let cost_factor = 1.0 / (1.0 + cost_per_1k_tokens.max(0.01));
    (efficiency * 0.75) + (cost_factor * 0.25)
}

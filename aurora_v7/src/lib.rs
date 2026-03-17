pub mod agents;
pub mod events;
pub mod manager;
pub mod memory;
pub mod models;
pub mod router;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::agents::{AutomationAgent, MediaAgent, SecurityAgent};
    use crate::manager::AgentAndModelManager;
    use crate::memory::MultimodalMemorySystem;
    use crate::models::{ModelCapability, StaticModelProvider};
    use crate::router::IntelligentTaskRouter;
    use crate::types::{Task, TaskType};

    fn setup_manager() -> AgentAndModelManager {
        let mut manager = AgentAndModelManager::new();

        manager.register_agent(Box::new(SecurityAgent::new("sec-core")));
        manager.register_agent(Box::new(MediaAgent::new("media-lab")));
        manager.register_agent(Box::new(AutomationAgent::new("auto-flow")));

        manager.register_model("deepseek", vec![ModelCapability::Reasoning], 0.18, 0.94);
        manager.register_model(
            "qwen-3.5-vision",
            vec![ModelCapability::Vision, ModelCapability::ImageGeneration],
            0.22,
            0.9,
        );

        manager
    }

    #[test]
    fn routes_logic_task_to_deepseek() {
        let manager = setup_manager();
        let provider = StaticModelProvider::new(manager.models().clone());
        let router = IntelligentTaskRouter::new(manager, provider);
        let mut memory = MultimodalMemorySystem::new();

        let task = Task::new(
            "Analiza las dependencias y encuentra ciclos lógicos",
            TaskType::Reasoning,
        );

        let decision = router.route(&task, &mut memory).expect("routing works");
        assert_eq!(decision.selected_model, "deepseek");
        assert_eq!(decision.selected_agent, "sec-core");
    }

    #[test]
    fn routes_image_task_to_qwen() {
        let manager = setup_manager();
        let provider = StaticModelProvider::new(manager.models().clone());
        let router = IntelligentTaskRouter::new(manager, provider);
        let mut memory = MultimodalMemorySystem::new();

        let task = Task::new(
            "Genera keyframes para un anuncio de producto",
            TaskType::ImageGeneration,
        );

        let decision = router.route(&task, &mut memory).expect("routing works");
        assert_eq!(decision.selected_model, "qwen-3.5-vision");
        assert_eq!(decision.selected_agent, "media-lab");
    }
}

use aurora_v7::agents::{AutomationAgent, MediaAgent, SecurityAgent};
use aurora_v7::manager::AgentAndModelManager;
use aurora_v7::memory::MultimodalMemorySystem;
use aurora_v7::models::{ModelCapability, StaticModelProvider};
use aurora_v7::router::IntelligentTaskRouter;
use aurora_v7::types::{Task, TaskType};

fn main() {
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

    let provider = StaticModelProvider::new(manager.models().clone());
    let router = IntelligentTaskRouter::new(manager, provider);
    let mut memory = MultimodalMemorySystem::new();

    let logical_task = Task::new(
        "Resolver árbol de decisión y hallar contradicciones",
        TaskType::Reasoning,
    );
    let image_task = Task::new(
        "Diseñar escena hero para campaña urbana",
        TaskType::ImageGeneration,
    );

    let logic_decision = router.route(&logical_task, &mut memory).unwrap();
    let image_decision = router.route(&image_task, &mut memory).unwrap();

    println!(
        "Lógica -> agente={}, modelo={}",
        logic_decision.selected_agent, logic_decision.selected_model
    );
    println!(
        "Imagen -> agente={}, modelo={}",
        image_decision.selected_agent, image_decision.selected_model
    );
}

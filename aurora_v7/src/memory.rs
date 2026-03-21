use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub trait MemoryStore: Send + Sync {
    fn save(&mut self, namespace: &str, key: &str, value: &str);
    fn get(&self, namespace: &str, key: &str) -> Option<String>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MultimodalMemorySystem {
    pub database: HashMap<String, String>,
    pub vector_memory: HashMap<String, Vec<f32>>,
    pub asset_library: HashMap<String, String>,
    pub knowledge_graph: HashMap<String, Vec<String>>,
}

impl MultimodalMemorySystem {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn save_embedding(&mut self, concept: &str, embedding: Vec<f32>) {
        self.vector_memory.insert(concept.to_string(), embedding);
    }

    pub fn save_asset_version(&mut self, asset_id: &str, uri: &str) {
        self.asset_library
            .insert(asset_id.to_string(), uri.to_string());
    }

    pub fn link_entities(&mut self, source: &str, target: &str) {
        self.knowledge_graph
            .entry(source.to_string())
            .or_default()
            .push(target.to_string());
    }
}

impl MemoryStore for MultimodalMemorySystem {
    fn save(&mut self, namespace: &str, key: &str, value: &str) {
        self.database
            .insert(format!("{namespace}:{key}"), value.to_string());
    }

    fn get(&self, namespace: &str, key: &str) -> Option<String> {
        self.database.get(&format!("{namespace}:{key}")).cloned()
    }
}

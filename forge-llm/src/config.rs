use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct LlmConfig {
    pub model_path: String,
    pub context_size: usize,
    pub threads: usize,
    pub gpu_layers: usize,
    pub bind_address: String,
    pub bind_port: u16,
    pub temperature: f32,
    pub top_p: f32,
    pub max_tokens: usize,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            model_path: "/opt/forge/models/forge-v1.gguf".into(),
            context_size: 8192, threads: 4, gpu_layers: 0,
            bind_address: "127.0.0.1".into(), bind_port: 8080,
            temperature: 0.7, top_p: 0.95, max_tokens: 2048,
        }
    }
}

pub fn load() -> anyhow::Result<LlmConfig> {
    let path = "/etc/forge/llm.json";
    if let Ok(data) = std::fs::read_to_string(path) {
        Ok(serde_json::from_str(&data)?)
    } else {
        Ok(LlmConfig::default())
    }
}

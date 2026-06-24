use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;
use tracing::info;
use crate::config::LlmConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage { pub role: String, pub content: String, pub images: Option<Vec<String>> }

#[derive(Debug, Clone)]
pub struct GenerationParams { pub temperature: f32, pub top_p: f32, pub max_tokens: usize }

impl Default for GenerationParams {
    fn default() -> Self { Self { temperature: 0.7, top_p: 0.95, max_tokens: 2048 } }
}

#[derive(Debug, Clone)]
pub struct GenerationResult { pub text: String, pub tokens_generated: usize, pub tokens_per_sec: f64, pub finish_reason: String }

pub struct InferenceEngine {
    model_path: String,
    llama_cli: String,
    config: LlmConfig,
}

impl InferenceEngine {
    pub fn new(config: &LlmConfig) -> Result<Self> {
        let llama_cli = find_llama_cli();
        let model_exists = Path::new(&config.model_path).exists();

        if model_exists {
            info!("GGUF model found: {}", config.model_path);
        } else {
            info!("GGUF model not found. Download one to: {}", config.model_path);
        }
        info!("llama.cpp CLI: {}", llama_cli);

        Ok(Self {
            model_path: config.model_path.clone(),
            llama_cli,
            config: config.clone(),
        })
    }

    pub fn backend_name(&self) -> &str {
        if Path::new(&self.model_path).exists() && Path::new(&self.llama_cli).exists() {
            "llama.cpp (GGUF)"
        } else {
            "stub (no model/cli)"
        }
    }

    pub fn chat(&self, messages: &[ChatMessage], params: &GenerationParams) -> GenerationResult {
        if !Path::new(&self.model_path).exists() || !Path::new(&self.llama_cli).exists() {
            return self.chat_stub(messages);
        }

        let prompt = Self::build_prompt(messages);
        let start = std::time::Instant::now();

        match Command::new(&self.llama_cli)
            .args([
                "-m", &self.model_path,
                "-p", &prompt,
                "-n", &params.max_tokens.to_string(),
                "--temp", &params.temperature.to_string(),
                "--top-p", &params.top_p.to_string(),
                "-c", &self.config.context_size.to_string(),
                "-t", &self.config.threads.to_string(),
                "--no-display-prompt",
                "--simple-io",
            ])
            .output()
        {
            Ok(output) => {
                let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let elapsed = start.elapsed().as_secs_f64();
                let tokens = text.split_whitespace().count();
                GenerationResult {
                    text,
                    tokens_generated: tokens,
                    tokens_per_sec: if elapsed > 0.0 { tokens as f64 / elapsed } else { 0.0 },
                    finish_reason: "stop".into(),
                }
            }
            Err(e) => {
                GenerationResult {
                    text: format!("LLM error: {e}"),
                    tokens_generated: 0,
                    tokens_per_sec: 0.0,
                    finish_reason: "error".into(),
                }
            }
        }
    }

    fn build_prompt(messages: &[ChatMessage]) -> String {
        let mut prompt = String::from("<|system|>\n你是 Forge OS AI 助手。用中文简洁回复。\n");
        for msg in messages {
            match msg.role.as_str() {
                "user" => { prompt.push_str(&format!("<|user|>\n{}\n", msg.content)); }
                "assistant" => { prompt.push_str(&format!("<|assistant|>\n{}\n", msg.content)); }
                "system" => { prompt.push_str(&format!("<|system|>\n{}\n", msg.content)); }
                _ => {}
            }
        }
        prompt.push_str("<|assistant|>\n");
        prompt
    }

    fn chat_stub(&self, messages: &[ChatMessage]) -> GenerationResult {
        let last = messages.last().map(|m| m.content.as_str()).unwrap_or("");
        let response = if last.contains("你好") || last.to_lowercase().contains("hello") {
            "你好！Forge OS 已就绪。请安装 GGUF 模型以获得智能回复。".to_string()
        } else {
            format!("[Forge stub] 收到: \"{}\". 安装 llama.cpp 和 GGUF 模型后启用。", last)
        };
        GenerationResult { text: response, tokens_generated: 0, tokens_per_sec: 0.0, finish_reason: "stub".into() }
    }
}

fn find_llama_cli() -> String {
    for path in &["/usr/bin/llama-cli", "/usr/local/bin/llama-cli", "/opt/forge/llama.cpp/build/bin/llama-cli"] {
        if std::path::Path::new(path).exists() {
            return path.to_string();
        }
    }
    "llama-cli".to_string()
}

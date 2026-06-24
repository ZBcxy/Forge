use anyhow::Result;
use std::path::Path;
use std::process::Command;
use tracing::info;
use crate::config::LlmConfig;
use crate::gpu::GpuConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage { pub role: String, pub content: String }

#[derive(Debug, Clone)]
pub struct GenerationParams { pub temperature: f32, pub top_p: f32, pub max_tokens: usize }

impl Default for GenerationParams {
    fn default() -> Self { Self { temperature: 0.7, top_p: 0.95, max_tokens: 2048 } }
}

#[derive(Debug, Clone)]
pub struct GenerationResult { pub text: String, pub tokens_generated: usize, pub finish_reason: String }

pub struct InferenceEngine {
    model_path: String,
    llama_cli: String,
    config: LlmConfig,
    gpu: GpuConfig,
}

impl InferenceEngine {
    pub fn new(config: &LlmConfig, gpu: &GpuConfig) -> Result<Self> {
        let llama_cli = find_llama_cli();
        let model_exists = Path::new(&config.model_path).exists();
        if model_exists { info!("Model: {}", config.model_path); }
        else { info!("No model found. Run forge-model-dl after boot."); }
        Ok(Self { model_path: config.model_path.clone(), llama_cli, config: config.clone(), gpu: gpu.clone() })
    }

    pub fn backend_name(&self) -> &str {
        if self.gpu.enabled {
            match self.gpu.backend {
                crate::gpu::GpuBackend::Vulkan => "llama.cpp + Vulkan",
                crate::gpu::GpuBackend::Cuda => "llama.cpp + CUDA",
                crate::gpu::GpuBackend::Metal => "llama.cpp + Metal",
                crate::gpu::GpuBackend::CpuOnly => "llama.cpp (CPU)",
            }
        } else {
            "llama.cpp (CPU)"
        }
    }

    pub fn chat(&self, messages: &[ChatMessage], params: &GenerationParams) -> GenerationResult {
        if !Path::new(&self.model_path).exists() || !Path::new(&self.llama_cli).exists() {
            return self.stub(messages);
        }
        let prompt = build_prompt(messages);
        let n_str = params.max_tokens.to_string(); let temp_str = params.temperature.to_string(); let top_p_str = params.top_p.to_string(); let ctx_str = self.config.context_size.to_string(); let threads_str = self.config.threads.to_string(); let mut args = vec![
            "-m", &self.model_path, "-p", &prompt,
            "-n", &n_str,
            "--temp", &temp_str,
            "--top-p", &top_p_str,
            "-c", &ctx_str,
            "-t", &threads_str,
            "--no-display-prompt", "--simple-io",
        ];
        let gpu_args = self.gpu.to_cli_args();
        for a in &gpu_args { args.push(a); }

        match Command::new(&self.llama_cli).args(&args).output() {
            Ok(o) => {
                let text = String::from_utf8_lossy(&o.stdout).trim().to_string();
                let n = text.split_whitespace().count();
                GenerationResult { text, tokens_generated: n, finish_reason: "stop".into() }
            }
            Err(e) => GenerationResult { text: format!("Error: {e}"), tokens_generated: 0, finish_reason: "error".into() }
        }
    }

    fn stub(&self, messages: &[ChatMessage]) -> GenerationResult {
        let last = messages.last().map(|m| m.content.as_str()).unwrap_or("");
        GenerationResult { text: format!("[Forge v0.3.0 | {}]\nInstall a GGUF model for AI replies. Run: forge-model-dl", self.backend_name()), tokens_generated: 0, finish_reason: "stub".into() }
    }
}

fn build_prompt(msgs: &[ChatMessage]) -> String {
    let mut p = String::from("<|system|>\nYou are Forge OS AI assistant. Reply concisely.\n");
    for m in msgs {
        match m.role.as_str() {
            "user" => { p.push_str(&format!("<|user|>\n{}\n", m.content)); }
            "assistant" => { p.push_str(&format!("<|assistant|>\n{}\n", m.content)); }
            _ => {}
        }
    }
    p.push_str("<|assistant|>\n");
    p
}

fn find_llama_cli() -> String {
    for p in &["/usr/bin/llama-cli", "/usr/local/bin/llama-cli", "/opt/forge/llama.cpp/build/bin/llama-cli"] {
        if std::path::Path::new(p).exists() { return p.to_string(); }
    }
    "llama-cli".to_string()
}
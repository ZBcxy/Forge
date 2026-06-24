mod engine;
mod api;
mod vision;
mod config;
mod gpu;

use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).with_ansi(false)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).init();

    info!("Forge LLM inference engine starting...");
    let cfg = config::load()?;
    info!("Model: {} | ctx: {} tokens", cfg.model_path, cfg.context_size);

    let gpu = gpu::GpuConfig::detect();
    info!("GPU: {:?} | VRAM: {} MB | layers: {} | enabled: {}",
        gpu.backend, gpu.vram_mb, gpu.gpu_layers, gpu.enabled);

    let engine = engine::InferenceEngine::new(&cfg, &gpu)?;
    info!("Backend: {}", engine.backend_name());

    let addr = format!("{}:{}", cfg.bind_address, cfg.bind_port);
    info!("HTTP API on {}", addr);
    api::serve(engine, addr).await?;
    Ok(())
}
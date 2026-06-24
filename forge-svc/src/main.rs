// Forge Service Manager — supervises all Forge OS daemons

mod service;
mod config;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use service::{ServiceState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).with_ansi(false).init();
    info!("Forge Service Manager starting — PID {}", std::process::id());

    let services = config::load_services();
    let state = Arc::new(RwLock::new(ServiceState::new(services)));

    start_in_order(&state).await;
    monitor_loop(state).await;
    Ok(())
}

async fn start_in_order(state: &Arc<RwLock<ServiceState>>) {
    for name in &["forge-hal", "forge-llm", "forge-voice", "forge-agent", "forge-gui"] {
        info!("Starting: {name}");
        if let Err(e) = state.write().await.start_service(name).await {
            error!("Start {name} failed: {e}");
        }
    }
}

async fn monitor_loop(state: Arc<RwLock<ServiceState>>) {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
        state.write().await.check_and_restart().await;
    }
}

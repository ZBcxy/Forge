mod ooda;
mod tools;
mod memory;

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use ooda::OodaAgent;

#[derive(Clone)]
struct AppState { agent: Arc<Mutex<OodaAgent>> }

#[derive(Debug, Deserialize)]
struct CommandRequest { text: String, source: Option<String> }

#[derive(Debug, Serialize)]
struct CommandResponse { reply: String, action: String, success: bool }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).with_ansi(false).init();
    info!("Forge Agent starting...");

    let agent = OodaAgent::new().await?;
    let state = AppState { agent: Arc::new(Mutex::new(agent)) };

    let app = Router::new()
        .route("/health", axum::routing::get(|| async { Json(serde_json::json!({"ok":true,"tools":20})) }))
        .route("/command", post(handle_command))
        .with_state(state);

    let bind = std::env::var("FORGE_AGENT_BIND").unwrap_or_else(|_| "127.0.0.1:8081".into());
    info!("Agent API on {bind} (20 tools ready)");

    let listener = tokio::net::TcpListener::bind(&bind).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_command(State(state): State<AppState>, Json(req): Json<CommandRequest>) -> Json<CommandResponse> {
    let mut agent = state.agent.lock().await;
    let result = agent.process(&req.text).await;
    Json(CommandResponse { reply: result.reply, action: result.action, success: result.success })
}

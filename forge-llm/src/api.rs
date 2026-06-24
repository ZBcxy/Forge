use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::Deserialize;
use std::sync::Arc;
use tracing::info;
use crate::engine::{ChatMessage, GenerationParams, InferenceEngine};

#[derive(Clone)]
struct AppState { engine: Arc<InferenceEngine> }

#[derive(Debug, Deserialize)]
struct ChatRequest { messages: Vec<ChatMessage>, temperature: Option<f32>, max_tokens: Option<usize> }

pub async fn serve(engine: InferenceEngine, bind_addr: String) -> anyhow::Result<()> {
    let state = AppState { engine: Arc::new(engine) };
    let app = Router::new()
        .route("/health", get(|| async { Json(serde_json::json!({"status":"ok","service":"forge-llm"})) }))
        .route("/v1/chat", post(|State(state): State<AppState>, Json(req): Json<ChatRequest>| async move {
            let params = GenerationParams {
                temperature: req.temperature.unwrap_or(0.7),
                max_tokens: req.max_tokens.unwrap_or(2048),
                ..Default::default()
            };
            let result = state.engine.chat(&req.messages, &params);
            Json(serde_json::json!({"text": result.text, "tokens": result.tokens_generated, "finish": result.finish_reason}))
        }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

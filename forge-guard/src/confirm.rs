use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmRequest {
    pub operation: String,
    pub target: String,
    pub risk_level: RiskLevel,
    pub challenge: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RiskLevel { Low, Medium, High, Critical }

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfirmResponse {
    pub approved: bool,
    pub reason: Option<String>,
}

const CRITICAL_OPS: &[&str] = &["sys_shutdown", "sys_reboot", "fs_delete", "fs_move", "proc_kill"];

pub async fn handle_confirmation(mut socket: TcpStream) {
    let mut buf = vec![0u8; 4096];
    let n = socket.read(&mut buf).await.unwrap_or(0);
    if n == 0 { return; }
    let req: ConfirmRequest = match serde_json::from_slice(&buf[..n]) {
        Ok(r) => r,
        Err(_) => { return; }
    };
    let approved = match req.risk_level {
        RiskLevel::Low => true,
        RiskLevel::Medium => true,
        RiskLevel::High => false,
        RiskLevel::Critical => false,
    };
    let resp = ConfirmResponse { approved, reason: None };
    let _ = socket.write_all(&serde_json::to_vec(&resp).unwrap()).await;
}

pub fn classify_risk(op: &str) -> RiskLevel {
    if CRITICAL_OPS.contains(&op) { RiskLevel::Critical }
    else if op.starts_with("fs_") { RiskLevel::Medium }
    else if op.starts_with("net_") { RiskLevel::High }
    else { RiskLevel::Low }
}
use serde::Serialize;
use std::fs::OpenOptions;
use std::io::Write;
#[derive(Debug, Serialize)]
pub struct AuditEntry { pub timestamp: String, pub user: String, pub action: String, pub target: String, pub result: String, pub ip: String, pub risk: String }
pub fn log(entry: &AuditEntry) {
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open("/var/log/forge/audit.jsonl") {
        let _ = writeln!(f, "{}", serde_json::to_string(entry).unwrap_or_default());
    }
}
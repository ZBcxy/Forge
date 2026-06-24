use std::path::Path;
use tokio::net::UnixListener;
use tokio::io::AsyncWriteExt;
use tracing::info;

const SOCK: &str = "/run/forge/hal.sock";

pub async fn serve() -> anyhow::Result<()> {
    if let Some(p) = Path::new(SOCK).parent() { std::fs::create_dir_all(p)?; }
    let _ = std::fs::remove_file(SOCK);
    let listener = UnixListener::bind(SOCK)?;
    info!("HAL IPC on {SOCK}");
    loop {
        let (mut stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            let caps = serde_json::json!({"status":"ok","audio":{"mics":1,"speakers":1},"camera":{"count":0},"gpu":{"backend":"cpu","accel":false}});
            let _ = stream.write_all(&serde_json::to_vec(&caps).unwrap()).await;
        });
    }
}

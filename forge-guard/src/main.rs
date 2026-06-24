mod auth;
mod confirm;
mod monitor;
mod lock;
mod audit;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tokio::spawn(auth::run_auth_daemon());
    tokio::spawn(monitor::run_monitor());
    tokio::spawn(lock::run_auto_lock());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8099").await?;
    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(confirm::handle_confirmation(socket));
    }
}
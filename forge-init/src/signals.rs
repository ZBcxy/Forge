use std::sync::atomic::{AtomicBool, Ordering};
use tracing::info;

static SHUTDOWN: AtomicBool = AtomicBool::new(false);

pub fn setup_handlers() { unsafe { libc::signal(libc::SIGCHLD, libc::SIG_DFL); } }

pub fn should_shutdown() -> bool { SHUTDOWN.load(Ordering::Relaxed) }

pub fn request_shutdown() {
    info!("Shutdown requested");
    SHUTDOWN.store(true, Ordering::Relaxed);
}

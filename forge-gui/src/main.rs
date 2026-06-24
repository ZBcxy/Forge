mod compositor;
mod theme;
mod widget;
mod community;
mod protocol;
mod font;

use tracing::info;
use compositor::{Compositor, GuiState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).with_ansi(false).init();
    info!("Forge GUI v1.2 starting...");
    let theme = theme::load_active_theme()?;
    info!("Theme: {}", theme.meta.name);
    let themes = community::scan_installed()?;
    info!("Community themes: {}", themes.len());
    let state = GuiState::default();
    let mut c = Compositor::new(1280, 720, theme);
    info!("Windows: {}", state.windows.len());
    for w in &state.windows { info!("  {} - {}", w.title, w.kind.name()); }
    let _frame = c.render_frame(&state);
    loop { tokio::time::sleep(tokio::time::Duration::from_secs(60)).await; }
}
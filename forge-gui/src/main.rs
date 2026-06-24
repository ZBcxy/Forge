mod compositor;
mod theme;
mod widget;
mod community;
mod protocol;

use tracing::info;

use compositor::{Compositor, GuiState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).with_ansi(false).init();
    info!("Forge GUI starting...");

    let theme = theme::load_active_theme()?;
    info!("Theme: {} by {}", theme.meta.name, theme.meta.author);

    let community_themes = community::scan_installed()?;
    info!("Community themes: {} installed", community_themes.len());

    let state = GuiState::default();
    let mut compositor = Compositor::new(1280, 720, theme);

    info!("Compositor initialized (1280x720 framebuffer)");

    // Render loop — in production, this runs at 60fps with a window
    // For headless/embedded mode, we render on state change
    let _frame = compositor.render_frame(&state);

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
    }
}
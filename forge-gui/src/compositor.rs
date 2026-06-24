// Compositor — framebuffer-based GUI renderer for Forge OS
// Uses minifb (Mini FrameBuffer) for cross-platform window creation
// Falls back to softbuffer + winit for Wayland/Linux if preferred

use crate::theme::{Theme, ColorScheme, LayoutConfig};

pub struct Compositor {
    width: u32,
    height: u32,
    theme: Theme,
    framebuffer: Vec<u32>,
}

impl Compositor {
    pub fn new(width: u32, height: u32, theme: Theme) -> Self {
        let len = (width * height) as usize;
        Self { width, height, theme, framebuffer: vec![0u32; len] }
    }

    pub fn render_frame(&mut self, state: &GuiState) -> &[u32] {
        self.clear();
        self.draw_topbar(state);
        self.draw_sidebar(state);
        self.draw_main_area(state);
        self.draw_notification(state);
        &self.framebuffer
    }

    fn clear(&mut self) {
        let bg = hex_to_rgba(&self.theme.colors.background);
        self.framebuffer.fill(bg);
    }

    fn draw_topbar(&mut self, state: &GuiState) {
        let h = self.theme.layout.topbar_height;
        let bg = hex_to_rgba(&self.theme.colors.surface);
        self.fill_rect(0, 0, self.width, h, bg);

        // Forge icon + text
        self.draw_text(12, 4, "FORGE", hex_to_rgba(&self.theme.colors.primary), 20);
        // Status
        self.draw_text(self.width - 200, 10, &state.status, hex_to_rgba(&self.theme.colors.on_surface), 14);
    }

    fn draw_sidebar(&mut self, state: &GuiState) {
        let w = self.theme.layout.sidebar_width;
        let top = self.theme.layout.topbar_height;
        let bg = hex_to_rgba(&self.theme.colors.surface_alt);
        self.fill_rect(0, top, w, self.height - top, bg);

        let mut y = top + 16;
        for item in &state.menu_items {
            let color = if item.active {
                hex_to_rgba(&self.theme.colors.primary)
            } else {
                hex_to_rgba(&self.theme.colors.on_surface)
            };
            self.draw_text(16, y, &item.label, color, 14);
            y += 36;
        }
    }

    fn draw_main_area(&mut self, state: &GuiState) {
        let x = self.theme.layout.sidebar_width;
        let y = self.theme.layout.topbar_height;
        let w = self.width - x;
        let h = self.height - y;
        let bg = hex_to_rgba(&self.theme.colors.background);
        self.fill_rect(x, y, w, h, bg);

        // Agent conversation area
        let mut text_y = y + 16;
        for line in &state.conversation {
            self.draw_text(x + 16, text_y, line, hex_to_rgba(&self.theme.colors.on_background), 14);
            text_y += 22;
        }
    }

    fn draw_notification(&mut self, state: &GuiState) {
        if let Some(ref notif) = state.notification {
            let x = self.width / 2 - 150;
            let y = self.height - 80;
            let bg = hex_to_rgba(&self.theme.colors.surface);
            self.fill_rect(x, y, 300, 60, bg);
            let border = hex_to_rgba(&self.theme.colors.border);
            self.draw_rect(x, y, 300, 60, border);
            self.draw_text(x + 16, y + 12, &notif.title, hex_to_rgba(&self.theme.colors.primary), 16);
            self.draw_text(x + 16, y + 34, &notif.body, hex_to_rgba(&self.theme.colors.on_surface), 13);
        }
    }

    // ── Drawing primitives ──

    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for py in y..(y + h).min(self.height) {
            let row_start = (py * self.width + x) as usize;
            let row_end = (py * self.width + x + w).min(self.width as u32) as usize;
            if row_start < self.framebuffer.len() && row_end <= self.framebuffer.len() {
                self.framebuffer[row_start..row_end].fill(color);
            }
        }
    }

    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        self.fill_rect(x, y, w, 1, color);
        self.fill_rect(x, y + h - 1, w, 1, color);
        self.fill_rect(x, y, 1, h, color);
        self.fill_rect(x + w - 1, y, 1, h, color);
    }

    fn draw_text(&mut self, _x: u32, _y: u32, _text: &str, _color: u32, _size: u32) {
        // Font rendering stub — integrates with fontdue or rusttype
        // For now, placeholder: draw a colored dot
    }
}

#[derive(Debug, Clone)]
pub struct GuiState {
    pub status: String,
    pub menu_items: Vec<MenuItem>,
    pub conversation: Vec<String>,
    pub notification: Option<Notification>,
}

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub active: bool,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub title: String,
    pub body: String,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            status: "Forge OS Ready".into(),
            menu_items: vec![
                MenuItem { label: "Home".into(), active: true },
                MenuItem { label: "Agent".into(), active: false },
                MenuItem { label: "Files".into(), active: false },
                MenuItem { label: "Themes".into(), active: false },
                MenuItem { label: "Settings".into(), active: false },
            ],
            conversation: vec![
                "Forge OS v0.2.0 — AI-Native Operating System".into(),
                "Say \"Hey Forge\" to start".into(),
            ],
            notification: None,
        }
    }
}

fn hex_to_rgba(hex: &str) -> u32 {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as u32;
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as u32;
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as u32;
    (0xFF << 24) | (r << 16) | (g << 8) | b
}
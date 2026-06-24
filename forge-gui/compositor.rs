use crate::theme::Theme;
use tracing;

pub struct FontRenderer {
    font: Option<fontdue::Font>,
}

const FONT_PATHS: &[&str] = &[
    "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
    "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
    "/usr/share/fonts/TTF/DejaVuSans.ttf",
];

impl FontRenderer {
    pub fn new() -> Self {
        let font = FONT_PATHS.iter().find_map(|path| {
            std::fs::read(path).ok().and_then(|data| {
                fontdue::Font::from_bytes(&data, fontdue::FontSettings::default()).ok()
            })
        });
        if font.is_none() { tracing::warn!("No system font found"); }
        Self { font }
    }

    pub fn rasterize(&self, ch: char, size: f32) -> Option<(fontdue::Metrics, Vec<u8>)> {
        self.font.as_ref().map(|f| f.rasterize(ch, size))
    }
}

pub struct Compositor {
    width: u32, height: u32, theme: Theme,
    framebuffer: Vec<u32>, font: FontRenderer,
}

impl Compositor {
    pub fn new(width: u32, height: u32, theme: Theme) -> Self {
        let len = (width * height) as usize;
        Self { width, height, theme, framebuffer: vec![0u32; len], font: FontRenderer::new() }
    }

    pub fn render_frame(&mut self, state: &GuiState) -> &[u32] {
        self.clear(); self.draw_windows(state); self.draw_topbar(state); self.draw_notification(state);
        &self.framebuffer
    }

    fn clear(&mut self) { self.framebuffer.fill(hex(&self.theme.colors.background)); }

    fn draw_windows(&mut self, state: &GuiState) {
        for w in &state.windows {
            if !w.visible { continue; }
            self.fill_rect(w.x, w.y, w.width, w.height, hex(&self.theme.colors.surface));
            self.draw_rect(w.x, w.y, w.width, w.height, hex(&self.theme.colors.border));
            self.fill_rect(w.x, w.y, w.width, 32, hex(&self.theme.colors.surface_alt));
            self.draw_text(w.x + 12, w.y + 6, &w.title, hex(&self.theme.colors.on_surface), 16);
            if w.focused { self.fill_rect(w.x - 1, w.y - 1, w.width + 2, w.height + 2, hex(&self.theme.colors.primary)); }
            let mut cy = w.y + 40;
            for line in &w.content { self.draw_text(w.x + 12, cy, line, hex(&self.theme.colors.on_background), 14); cy += 22; }
        }
    }

    fn draw_topbar(&mut self, state: &GuiState) {
        self.fill_rect(0, 0, self.width, self.theme.layout.topbar_height, hex(&self.theme.colors.surface));
        self.draw_text(12, 8, &state.status, hex(&self.theme.colors.primary), 18);
    }

    fn draw_notification(&mut self, state: &GuiState) {
        if let Some(ref n) = state.notification {
            let x = self.width / 2 - 160; let y = self.height - 80;
            self.fill_rect(x, y, 320, 60, hex(&self.theme.colors.surface));
            self.draw_rect(x, y, 320, 60, hex(&self.theme.colors.border));
            self.draw_text(x + 16, y + 12, &n.title, hex(&self.theme.colors.primary), 16);
            self.draw_text(x + 16, y + 34, &n.body, hex(&self.theme.colors.on_surface), 13);
        }
    }

    fn fill_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        for py in y..(y+h).min(self.height) {
            let s = (py * self.width + x) as usize;
            let e = ((py * self.width).saturating_add(x).saturating_add(w).min(self.width as u32)) as usize;
            if s < self.framebuffer.len() && e <= self.framebuffer.len() { self.framebuffer[s..e].fill(color); }
        }
    }

    fn draw_rect(&mut self, x: u32, y: u32, w: u32, h: u32, color: u32) {
        self.fill_rect(x, y, w, 1, color); self.fill_rect(x, y+h-1, w, 1, color);
        self.fill_rect(x, y, 1, h, color); self.fill_rect(x+w-1, y, 1, h, color);
    }

    fn draw_text(&mut self, x: u32, y: u32, text: &str, color: u32, size: f32) {
        let Some(ref font) = self.font.font else { return; };
        let r = ((color>>16)&0xFF) as u8; let g = ((color>>8)&0xFF) as u8; let b = (color&0xFF) as u8;
        let mut cx = x;
        for ch in text.chars() {
            let (m, bm) = font.rasterize(ch, size);
            for (i, &a) in bm.iter().enumerate() {
                let bx = cx + (i as u32 % m.width);
                let by = y + (i as u32 / m.width);
                if bx < self.width && by < self.height {
                    let idx = (by * self.width + bx) as usize;
                    if idx < self.framebuffer.len() {
                        let alpha = a as u32;
                        let bg = self.framebuffer[idx];
                        let br = ((bg>>16)&0xFF)*(255-alpha)/255;
                        let bgg = ((bg>>8)&0xFF)*(255-alpha)/255;
                        let bb = (bg&0xFF)*(255-alpha)/255;
                        let fr = (r as u32*alpha)/255; let fg = (g as u32*alpha)/255; let fb = (b as u32*alpha)/255;
                        self.framebuffer[idx] = (0xFF<<24)|((br+fr)<<16)|((bgg+fg)<<8)|(bb+fb);
                    }
                }
            }
            cx += m.advance_width as u32;
        }
    }
}

#[derive(Debug, Clone)]
pub struct GuiState { pub status: String, pub windows: Vec<Window>, pub notification: Option<Notification> }
#[derive(Debug, Clone)]
pub struct Window { pub id: String, pub title: String, pub x: u32, pub y: u32, pub width: u32, pub height: u32, pub visible: bool, pub focused: bool, pub content: Vec<String>, pub kind: WindowKind }
#[derive(Debug, Clone, PartialEq)]
pub enum WindowKind { FileManager, Settings, Terminal, AgentChat, ThemePreview }
#[derive(Debug, Clone)]
pub struct Notification { pub title: String, pub body: String }

impl Default for GuiState {
    fn default() -> Self {
        Self {
            status: "Forge OS v0.3.0".into(),
            windows: vec![
                Window { id: "files".into(), title: "Files".into(), x: 270, y: 48, width: 400, height: 350, visible: true, focused: true, content: vec!["📁 /home/polaris/".into(),"  📄 README.md".into(),"  📁 Documents/".into(),"  📁 forge-os/".into()], kind: WindowKind::FileManager },
                Window { id: "settings".into(), title: "Settings".into(), x: 700, y: 48, width: 350, height: 300, visible: true, focused: false, content: vec!["⚙ Theme: Forge Default".into(),"🔊 Volume: 70%".into(),"🌐 Network: OK".into(),"💻 CPU: 2 cores".into(),"🧠 Agent: 20 tools".into()], kind: WindowKind::Settings },
            ],
            notification: None,
        }
    }
}

fn hex(h: &str) -> u32 { let h = h.trim_start_matches('#'); u32::from_str_radix(&h[..6], 16).unwrap_or(0) | 0xFF000000 }
use crate::theme::Theme;
use crate::font::BitmapFont;

pub struct Compositor {
    pub width: u32, pub height: u32,
    theme: Theme,
    framebuffer: Vec<u32>,
}

impl Compositor {
    pub fn new(width: u32, height: u32, theme: Theme) -> Self {
        Self { width, height, theme, framebuffer: vec![0u32; (width*height) as usize] }
    }

    pub fn render_frame(&mut self, state: &GuiState) -> &[u32] {
        self.clear();
        self.draw_windows(state);
        self.draw_topbar(state);
        self.draw_notification(state);
        &self.framebuffer
    }

    fn clear(&mut self) { self.framebuffer.fill(h(&self.theme.colors.background)); }

    fn draw_windows(&mut self, s: &GuiState) {
        for w in &s.windows {
            if !w.visible { continue; }
            self.fill(w.x, w.y, w.width, w.height, h(&self.theme.colors.surface));
            self.rect(w.x, w.y, w.width, w.height, h(&self.theme.colors.border));
            self.fill(w.x, w.y, w.width, 28, h(&self.theme.colors.surface_alt));
            self.text(w.x+10, w.y+6, &w.title, h(&self.theme.colors.on_surface));
            if w.focused { self.rect(w.x-1, w.y-1, w.width+2, w.height+2, h(&self.theme.colors.primary)); }
            let mut cy = w.y + 36;
            for line in &w.content { self.text(w.x+10, cy, line, h(&self.theme.colors.on_background)); cy += 20; }
        }
    }

    fn draw_topbar(&mut self, s: &GuiState) {
        self.fill(0, 0, self.width, self.theme.layout.topbar_height, h(&self.theme.colors.surface));
        self.text(14, 8, &s.status, h(&self.theme.colors.primary));
    }

    fn draw_notification(&mut self, s: &GuiState) {
        if let Some(ref n) = s.notification {
            let x = self.width/2 - 160; let y = self.height - 80;
            self.fill(x, y, 320, 60, h(&self.theme.colors.surface));
            self.rect(x, y, 320, 60, h(&self.theme.colors.border));
            self.text(x+16, y+12, &n.title, h(&self.theme.colors.primary));
            self.text(x+16, y+34, &n.body, h(&self.theme.colors.on_surface));
        }
    }

    fn fill(&mut self, x: u32, y: u32, w: u32, h: u32, c: u32) {
        for py in y..(y+h).min(self.height) {
            let start = (py as usize * self.width as usize + x as usize).min(self.framebuffer.len());
            let end = start.saturating_add(w as usize).min(self.framebuffer.len());
            if start < end { self.framebuffer[start..end].fill(c); }
        }
    }

    fn rect(&mut self, x: u32, y: u32, w: u32, h: u32, c: u32) {
        self.fill(x, y, w, 1, c); self.fill(x, y+h-1, w, 1, c);
        self.fill(x, y, 1, h, c); self.fill(x+w-1, y, 1, h, c);
    }

    fn text(&mut self, x: u32, y: u32, text: &str, color: u32) {
        let mut cx = x;
        let mut glyph_buf = [0u8; (BitmapFont::CHAR_W as usize * BitmapFont::CHAR_H as usize)];
        let cw = BitmapFont::CHAR_W;
        let ch = BitmapFont::CHAR_H;
        for chr in text.chars() {
            if chr == '\n' { continue; }
            BitmapFont::render_char(chr, &mut glyph_buf);
            for row in 0..ch as usize {
                for col in 0..cw as usize {
                    let alpha = glyph_buf[row * cw as usize + col];
                    if alpha == 0 { continue; }
                    let px = cx as usize + col;
                    let py = y as usize + row;
                    if px < self.width as usize && py < self.height as usize {
                        let bg = self.framebuffer[py * self.width as usize + px];
                        let a = alpha as f32 / 255.0;
                        let r = blend_ch((bg >> 16) as u8, (color >> 16) as u8, a);
                        let g = blend_ch((bg >> 8) as u8, (color >> 8) as u8, a);
                        let b = blend_ch(bg as u8, color as u8, a);
                        self.framebuffer[py * self.width as usize + px] = 0xFF000000 | ((r as u32) << 16) | ((g as u32) << 8) | b as u32;
                    }
                }
            }
            cx += cw;
        }
    }
}

fn blend_ch(bg: u8, fg: u8, alpha: f32) -> u8 {
    (fg as f32 * alpha + bg as f32 * (1.0 - alpha)) as u8
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
            status: "FORGE OS v1.2.0 -- AI-Native Operating System".into(),
            windows: vec![
                Window { id: "files".into(), title: "Files".into(), x: 270, y: 48, width: 420, height: 380, visible: true, focused: true, content: vec!["  /home/polaris/".into(),"    README.md".into(),"    Documents/".into(),"    Downloads/".into(),"    forge-os/".into(),"    projects/".into()], kind: WindowKind::FileManager },
                Window { id: "settings".into(), title: "Settings".into(), x: 710, y: 48, width: 360, height: 320, visible: true, focused: false, content: vec!["  Theme: Forge Default".into(),"  Volume: 70%".into(),"  Network: Connected".into(),"  CPU: 2 cores / 3.8 GB".into(),"  Guard: Active".into(),"  Agent: 20 tools ready".into()], kind: WindowKind::Settings },
            ],
            notification: None,
        }
    }
}

fn h(hex: &str) -> u32 {
    let h = hex.trim_start_matches('#');
    u32::from_str_radix(&h[..6.min(h.len())], 16).unwrap_or(0x0a0a0f) | 0xFF000000
}

impl WindowKind {
    pub fn name(&self) -> &str {
        match self {
            WindowKind::FileManager => "FileManager",
            WindowKind::Settings => "Settings",
            WindowKind::Terminal => "Terminal",
            WindowKind::AgentChat => "AgentChat",
            WindowKind::ThemePreview => "ThemePreview",
        }
    }
}
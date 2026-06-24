use crate::theme::Theme;

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

    fn text(&mut self, x: u32, y: u32, _text: &str, c: u32) {
        // Simple text indicator: draw a small colored bar per character
        let chars = _text.chars().count() as u32;
        let w = (chars * 8).min(300);
        self.fill(x, y + 12, w, 2, c);
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
            status: "FORGE OS v0.3.0 — AI-Native Operating System".into(),
            windows: vec![
                Window { id: "files".into(), title: "Files".into(), x: 270, y: 48, width: 420, height: 380, visible: true, focused: true, content: vec!["📁 /home/polaris/".into(),"  📄 README.md".into(),"  📁 Documents/".into(),"  📁 Downloads/".into(),"  📁 forge-os/".into(),"  📁 projects/".into()], kind: WindowKind::FileManager },
                Window { id: "settings".into(), title: "Settings".into(), x: 710, y: 48, width: 360, height: 320, visible: true, focused: false, content: vec!["⚙ Theme: Forge Default".into(),"🔊 Volume: 70%".into(),"🌐 Network: Connected".into(),"💻 CPU: 2 cores / 3.8 GB".into(),"🧠 Agent: 20 tools ready".into()], kind: WindowKind::Settings },
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
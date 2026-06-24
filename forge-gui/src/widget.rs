// Widget system — pluggable desktop widgets
//
// Widgets are small desktop utilities (clock, weather, system monitor, music player)
// that can be created by the community. Each widget is a self-contained Rust crate
// or Lua script.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
    pub id: String,
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub kind: WidgetKind,
    pub config: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetKind {
    Clock,
    Weather,
    SystemMonitor,
    MusicPlayer,
    TodoList,
    Calendar,
    Custom(String),
}

pub fn scan_widgets() -> Vec<Widget> {
    // Scan /opt/forge/widgets/ and ~/.forge/widgets/
    Vec::new()
}

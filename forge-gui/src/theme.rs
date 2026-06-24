// Theme engine — the heart of Forge GUI ecosystem
//
// Users can create, share, and install themes.
// Theme format: a directory with theme.json + assets/
// Community hub: /opt/forge/themes/community/

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tracing::info;

// ── Theme manifest ──

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub meta: ThemeMeta,
    pub colors: ColorScheme,
    pub fonts: FontConfig,
    pub layout: LayoutConfig,
    pub animations: AnimationConfig,
    pub sounds: SoundConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMeta {
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub homepage: Option<String>,
    pub license: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScheme {
    pub background: String,
    pub surface: String,
    pub surface_alt: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub danger: String,
    pub on_primary: String,
    pub on_background: String,
    pub on_surface: String,
    pub border: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontConfig {
    pub family: String,
    pub size: u32,
    pub weight: String,
    pub mono_family: String,
    pub mono_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutConfig {
    #[serde(rename = "padding")]
    pub padding_px: u32,
    pub radius_px: u32,
    pub gap_px: u32,
    pub sidebar_width: u32,
    pub topbar_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationConfig {
    pub enabled: bool,
    pub duration_ms: u32,
    pub easing: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundConfig {
    pub enabled: bool,
    pub volume: f32,
}

// ── Theme loading ──

const THEME_DIRS: &[&str] = &[
    "/opt/forge/themes/builtin/",
    "/opt/forge/themes/community/",
    "~/.forge/themes/",
];

pub fn load_active_theme() -> Result<Theme> {
    // Priority: user config > community default > builtin
    let config_path = "/etc/forge/gui.json";
    if let Ok(data) = std::fs::read_to_string(config_path) {
        if let Ok(cfg) = serde_json::from_str::<serde_json::Value>(&data) {
            if let Some(theme_name) = cfg["theme"].as_str() {
                for dir in THEME_DIRS {
                    let path = expand_path(&format!("{dir}/{theme_name}/theme.json"));
                    if let Ok(theme) = load_from_file(&path) {
                        info!("Loaded theme: {theme_name} from {dir}");
                        return Ok(theme);
                    }
                }
            }
        }
    }
    // Fallback to builtin default
    Ok(default_theme())
}

fn load_from_file(path: &str) -> Result<Theme> {
    let data = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&data)?)
}

pub fn default_theme() -> Theme {
    Theme {
        meta: ThemeMeta {
            name: "Forge Default".into(),
            author: "Forge Team".into(),
            version: "1.0.0".into(),
            description: "Default light/dark theme for Forge OS".into(),
            homepage: Some("https://github.com/ZBcxy/forge-os".into()),
            license: Some("Apache-2.0".into()),
        },
        colors: ColorScheme {
            background: "#0a0a0f".into(),
            surface: "#141420".into(),
            surface_alt: "#1a1a2e".into(),
            primary: "#6c63ff".into(),
            secondary: "#3b82f6".into(),
            accent: "#f59e0b".into(),
            danger: "#ef4444".into(),
            on_primary: "#ffffff".into(),
            on_background: "#e0e0e0".into(),
            on_surface: "#c0c0d0".into(),
            border: "#2a2a3a".into(),
        },
        fonts: FontConfig {
            family: "Inter".into(),
            size: 14,
            weight: "regular".into(),
            mono_family: "JetBrains Mono".into(),
            mono_size: 13,
        },
        layout: LayoutConfig {
            padding_px: 12,
            radius_px: 8,
            gap_px: 8,
            sidebar_width: 260,
            topbar_height: 40,
        },
        animations: AnimationConfig {
            enabled: true,
            duration_ms: 200,
            easing: "ease-out".into(),
        },
        sounds: SoundConfig {
            enabled: true,
            volume: 0.7,
        },
    }
}

pub fn expand_path(path: &str) -> String {
    if path.starts_with('~') {
        if let Ok(home) = std::env::var("HOME") {
            return path.replacen('~', &home, 1);
        }
    }
    path.into()
}


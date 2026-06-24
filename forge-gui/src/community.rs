// Community ecosystem — user-created themes, widgets, extensions
//
// Forge GUI is eco-friendly: users can design, share, and install:
// - Themes (color schemes, fonts, layouts, animations, sounds)
// - Widgets (desktop widgets written in Rust or Lua)
// - Extensions (plugins that add functionality)
//
// Community hub: https://forge-os.dev/community

use anyhow::Result;
use serde::{Deserialize, Serialize};

const COMMUNITY_DIR: &str = "/opt/forge/themes/community";
const USER_THEMES_DIR: &str = "~/.forge/themes";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityTheme {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub tags: Vec<String>,
    pub downloads: u64,
    pub rating: f32,
    pub installed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetDefinition {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub script: String,
    pub refresh_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionDefinition {
    pub id: String,
    pub name: String,
    pub author: String,
    pub version: String,
    pub description: String,
    pub entry: String,
    pub permissions: Vec<String>,
}

pub fn scan_installed() -> Result<Vec<CommunityTheme>> {
    let mut themes = Vec::new();
    let dirs = [COMMUNITY_DIR, USER_THEMES_DIR];
    for dir_str in &dirs {
        let dir = super::theme::expand_path(dir_str);
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let manifest = path.join("theme.json");
                    if manifest.exists() {
                        if let Ok(data) = std::fs::read_to_string(&manifest) {
                            if let Ok(theme) = serde_json::from_str::<super::theme::Theme>(&data) {
                                themes.push(CommunityTheme {
                                    id: path.file_name().unwrap().to_string_lossy().into(),
                                    name: theme.meta.name,
                                    author: theme.meta.author,
                                    version: theme.meta.version,
                                    description: theme.meta.description,
                                    tags: Vec::new(),
                                    downloads: 0,
                                    rating: 0.0,
                                    installed: true,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(themes)
}

pub fn install_theme_from_url(_url: &str) -> Result<()> {
    // FUTURE: Download theme archive, verify signature, extract to community dir
    Ok(())
}

pub fn create_theme_skeleton(name: &str, author: &str) -> Result<String> {
    let theme = super::theme::default_theme();
    let json = serde_json::to_string_pretty(&theme)?;
    let path = format!("/opt/forge/themes/community/{name}");
    std::fs::create_dir_all(&path)?;
    std::fs::write(format!("{path}/theme.json"), json)?;
    std::fs::create_dir_all(format!("{path}/assets"))?;

    let readme = format!("# {name}\n\nA Forge OS theme by {author}\n\n## License\n\nApache-2.0\n");
    std::fs::write(format!("{path}/README.md"), readme)?;

    Ok(path)
}

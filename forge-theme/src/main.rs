// forge-theme — Community Theme Manager for Forge OS
//
// Commands:
//   forge-theme list              List installed themes
//   forge-theme set <name>        Switch active theme
//   forge-theme inspect <path>    Validate and display a theme
//   forge-theme create <name>     Scaffold a new theme
//   forge-theme pack <path>       Package theme into .forge-theme archive
//   forge-theme install <file>    Install a .forge-theme package

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};

const COMMUNITY_DIR: &str = "/opt/forge/themes/community";
const USER_THEMES_DIR: &str = ".forge/themes";
const BUILTIN_DIR: &str = "/opt/forge/themes/builtin";
const GUI_CONFIG: &str = "/etc/forge/gui.json";

#[derive(Debug, Serialize, Deserialize)]
struct ThemeMeta {
    name: String,
    author: String,
    version: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ThemeManifest {
    meta: ThemeMeta,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        usage();
        return;
    }
    match args[1].as_str() {
        "list" => cmd_list(),
        "set" => cmd_set(args.get(2).map(|s| s.as_str()).unwrap_or("")),
        "inspect" => cmd_inspect(args.get(2).map(|s| s.as_str()).unwrap_or("")),
        "create" => cmd_create(args.get(2).map(|s| s.as_str()).unwrap_or("my-theme")),
        "pack" => cmd_pack(args.get(2).map(|s| s.as_str()).unwrap_or("")),
        "install" => cmd_install(args.get(2).map(|s| s.as_str()).unwrap_or("")),
        _ => usage(),
    }
}

fn usage() {
    println!("Forge Theme Manager");
    println!("  forge-theme list                  List installed themes");
    println!("  forge-theme set <name>            Switch active theme");
    println!("  forge-theme inspect <path>        Validate a theme");
    println!("  forge-theme create <name>         Scaffold a new theme");
    println!("  forge-theme pack <path>           Package to .forge-theme");
    println!("  forge-theme install <file>        Install a .forge-theme package");
}

fn cmd_list() {
    println!("Installed themes:\n");
    list_dir(BUILTIN_DIR, "(builtin)");
    list_dir(COMMUNITY_DIR, "(community)");
    list_user_themes();
}

fn list_dir(dir: &str, tag: &str) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                let path = entry.path().join("theme.json");
                if let Ok(data) = fs::read_to_string(&path) {
                    if let Ok(t) = serde_json::from_str::<ThemeManifest>(&data) {
                        println!("  {} {} v{} — {} {}", t.meta.name, tag, t.meta.version, t.meta.description, entry.path().display());
                    }
                }
            }
        }
    }
}

fn list_user_themes() {
    if let Ok(home) = env::var("HOME") {
        let ud = PathBuf::from(&home).join(USER_THEMES_DIR);
        if ud.exists() {
            list_dir(ud.to_str().unwrap(), "(user)");
        }
    }
}

fn cmd_set(name: &str) {
    if name.is_empty() {
        eprintln!("Usage: forge-theme set <theme-name>");
        return;
    }

    // Update GUI config
    let config_path = Path::new(GUI_CONFIG);
    let mut config: serde_json::Value = if config_path.exists() {
        serde_json::from_str(&fs::read_to_string(config_path).unwrap_or_default()).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };
    config["theme"] = serde_json::json!(name);

    let json = serde_json::to_string_pretty(&config).unwrap();
    // Write requires sudo for /etc; fall back to printing instructions
    if let Err(_) = fs::write(GUI_CONFIG, &json) {
        println!("Need sudo to write {GUI_CONFIG}. Run:");
        println!("  sudo forge-theme set \"{name}\"");
        println!("  Or manually edit {GUI_CONFIG} and set theme to \"{name}\"");
    } else {
        println!("Active theme set to \"{name}\"");
    }
}

fn cmd_inspect(path: &str) {
    if path.is_empty() {
        eprintln!("Usage: forge-theme inspect <path-to-theme-dir>");
        return;
    }
    let manifest = Path::new(path).join("theme.json");
    match fs::read_to_string(&manifest) {
        Ok(data) => match serde_json::from_str::<serde_json::Value>(&data) {
            Ok(v) => {
                println!("{}", serde_json::to_string_pretty(&v).unwrap());
                println!("\n✓ Valid theme.json");

                // Check for common issues
                if let Some(colors) = v.get("colors") {
                    for key in &["background","surface","surface_alt","primary","secondary","on_background","on_surface"] {
                        if colors.get(key).is_none() {
                            println!("⚠ Missing recommended color: {key}");
                        }
                    }
                }
                if v.get("meta").and_then(|m| m.get("name")).is_none() {
                    println!("⚠ Missing meta.name");
                }
                if Path::new(path).join("assets").exists() {
                    let count = fs::read_dir(Path::new(path).join("assets")).map(|d| d.count()).unwrap_or(0);
                    println!("✓ assets/ ({count} files)");
                }
            }
            Err(e) => eprintln!("✗ Invalid JSON: {e}"),
        },
        Err(e) => eprintln!("✗ Cannot read theme.json: {e}"),
    }
}

fn cmd_create(name: &str) {
    let theme_dir = PathBuf::from(name);
    if theme_dir.exists() {
        eprintln!("Directory '{}' already exists", name);
        return;
    }
    fs::create_dir_all(theme_dir.join("assets")).unwrap();
    fs::create_dir_all(theme_dir.join("assets/sounds")).unwrap();

    let theme = serde_json::json!({
        "meta": {
            "name": name,
            "author": "Your Name",
            "version": "1.0.0",
            "description": "A custom Forge OS theme",
            "license": "MIT"
        },
        "colors": {
            "background": "#0a0a0f",
            "surface": "#141420",
            "surface_alt": "#1a1a2e",
            "primary": "#6c63ff",
            "secondary": "#3b82f6",
            "accent": "#f59e0b",
            "danger": "#ef4444",
            "on_primary": "#ffffff",
            "on_background": "#e0e0e0",
            "on_surface": "#c0c0d0",
            "border": "#2a2a3a"
        },
        "fonts": {
            "family": "Inter", "size": 14, "weight": "regular",
            "mono_family": "JetBrains Mono", "mono_size": 13
        },
        "layout": {
            "padding": 12, "radius_px": 8, "gap_px": 8,
            "sidebar_width": 260, "topbar_height": 40
        },
        "animations": { "enabled": true, "duration_ms": 200, "easing": "ease-out" },
        "sounds": { "enabled": true, "volume": 0.7 }
    });

    let json = serde_json::to_string_pretty(&theme).unwrap();
    fs::write(theme_dir.join("theme.json"), &json).unwrap();

    let readme = format!("# {name}\n\nA custom Forge OS theme.\n\nEdit `theme.json` to customize colors, fonts, and layout.\n\nSee [THEME_SPEC.md](https://github.com/ZBcxy/forge-os/blob/main/community/THEME_SPEC.md) for documentation.\n");
    fs::write(theme_dir.join("README.md"), readme).unwrap();

    println!("Theme scaffolded: {}/", name);
    println!("  {name}/theme.json    ← edit colors here");
    println!("  {name}/assets/       ← add images, sounds");
    println!("  {name}/README.md     ← describe your theme");
    println!("\nNext: forge-theme inspect {name}   (validate)");
    println!("      forge-theme pack {name}       (package for sharing)");
}

fn cmd_pack(path: &str) {
    if path.is_empty() { eprintln!("Usage: forge-theme pack <theme-dir>"); return; }
    let dir = Path::new(path);
    if !dir.exists() || !dir.join("theme.json").exists() {
        eprintln!("No valid theme found at '{path}'");
        return;
    }

    let name = dir.file_name().unwrap().to_string_lossy();
    let archive = format!("{name}.forge-theme");

    let status = Command::new("tar")
        .args(["-czf", &archive, "-C", dir.parent().unwrap().to_str().unwrap_or("."), &name])
        .status();

    match status {
        Ok(s) if s.success() => println!("Packaged: {archive}"),
        _ => eprintln!("Failed to package. Make sure 'tar' is installed."),
    }
}

fn cmd_install(file: &str) {
    if file.is_empty() { eprintln!("Usage: forge-theme install <file.forge-theme>"); return; }
    if !Path::new(file).exists() {
        eprintln!("File not found: {file}");
        return;
    }

    // Install to community dir
    let dest = PathBuf::from(COMMUNITY_DIR);
    fs::create_dir_all(&dest).ok();

    let status = Command::new("tar")
        .args(["-xzf", file, "-C", dest.to_str().unwrap()])
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("Installed to {}/", COMMUNITY_DIR);
            println!("Run: forge-theme list     (see all themes)");
            println!("     forge-theme set ...  (activate)");
        }
        _ => {
            // Try user directory
            let home = env::var("HOME").unwrap_or_default();
            let user_dest = PathBuf::from(&home).join(USER_THEMES_DIR);
            fs::create_dir_all(&user_dest).ok();
            let status = Command::new("tar")
                .args(["-xzf", file, "-C", user_dest.to_str().unwrap()])
                .status();
            match status {
                Ok(s) if s.success() => println!("Installed to ~/.forge/themes/"),
                _ => eprintln!("Install failed. Try: tar -xzf {file} -C ~/.forge/themes/"),
            }
        }
    }
}
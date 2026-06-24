use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolCategory { FileSystem, Process, Network, System, Web, Display }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    pub success: bool,
    pub output: String,
    pub data: serde_json::Value,
}

pub struct ToolRegistry;

impl ToolRegistry {
    pub fn new() -> Self { Self }

    pub fn execute(&self, name: &str, args: &serde_json::Value) -> ToolResult {
        match name {
            // File system
            "fs_list"    => Self::fs_list(args),
            "fs_read"    => Self::fs_read(args),
            "fs_write"   => Self::fs_write(args),
            "fs_delete"  => Self::fs_delete(args),
            "fs_mkdir"   => Self::fs_mkdir(args),
            "fs_search"  => Self::fs_search(args),
            "fs_copy"    => Self::fs_copy(args),
            "fs_move"    => Self::fs_move(args),
            // Process
            "proc_launch" => Self::proc_launch(args),
            "proc_kill"   => Self::proc_kill(args),
            "proc_list"   => Self::proc_list(args),
            // System
            "sys_info"    => Self::sys_info(args),
            "sys_shutdown" => Self::sys_shutdown(args),
            "sys_reboot"  => Self::sys_reboot(args),
            "sys_volume"  => Self::sys_volume(args),
            // Network
            "net_http"    => Self::net_http(args),
            "net_download" => Self::net_download(args),
            // Web
            "web_search"  => Self::web_search(args),
            "web_browse"  => Self::web_browse(args),
            // Display
            "display_screenshot" => Self::display_screenshot(args),
            _ => ToolResult { success: false, output: format!("Unknown tool: {name}"), data: serde_json::json!(null) },
        }
    }

    fn fs_list(args: &serde_json::Value) -> ToolResult {
        let path = args["path"].as_str().unwrap_or("/home/polaris");
        match std::fs::read_dir(path) {
            Ok(entries) => {
                let items: Vec<String> = entries.filter_map(|e| {
                    e.ok().map(|e| {
                        let ft = if e.file_type().map(|t| t.is_dir()).unwrap_or(false) { "/" } else { "" };
                        format!("{}{ft}", e.file_name().to_string_lossy())
                    })
                }).collect();
                ToolResult { success: true, output: format!("{} items", items.len()), data: serde_json::json!(items) }
            }
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn fs_read(args: &serde_json::Value) -> ToolResult {
        let path = args["path"].as_str().unwrap_or("");
        match std::fs::read_to_string(path) {
            Ok(content) => ToolResult { success: true, output: format!("{} bytes", content.len()), data: serde_json::json!({"content": content}) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn fs_write(args: &serde_json::Value) -> ToolResult {
        let path = args["path"].as_str().unwrap_or("");
        let content = args["content"].as_str().unwrap_or("");
        match std::fs::write(path, content) {
            Ok(()) => ToolResult { success: true, output: "written".into(), data: serde_json::json!(null) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn fs_delete(args: &serde_json::Value) -> ToolResult {
        let path = args["path"].as_str().unwrap_or("");
        let result = if Path::new(path).is_dir() {
            std::fs::remove_dir_all(path)
        } else {
            std::fs::remove_file(path)
        };
        match result {
            Ok(()) => ToolResult { success: true, output: "deleted".into(), data: serde_json::json!(null) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn fs_mkdir(args: &serde_json::Value) -> ToolResult {
        let path = args["path"].as_str().unwrap_or("");
        match std::fs::create_dir_all(path) {
            Ok(()) => ToolResult { success: true, output: "created".into(), data: serde_json::json!(null) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn fs_search(args: &serde_json::Value) -> ToolResult {
        let dir = args["dir"].as_str().unwrap_or(".");
        let pattern = args["pattern"].as_str().unwrap_or("");
        let mut results = Vec::new();
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.contains(pattern) {
                    results.push(name);
                }
            }
        }
        ToolResult { success: true, output: format!("{} found", results.len()), data: serde_json::json!(results) }
    }

    fn fs_copy(args: &serde_json::Value) -> ToolResult {
        let src = args["src"].as_str().unwrap_or("");
        let dst = args["dst"].as_str().unwrap_or("");
        match std::fs::copy(src, dst) {
            Ok(n) => ToolResult { success: true, output: format!("{n} bytes"), data: serde_json::json!(null) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn fs_move(args: &serde_json::Value) -> ToolResult {
        let src = args["src"].as_str().unwrap_or("");
        let dst = args["dst"].as_str().unwrap_or("");
        match std::fs::rename(src, dst) {
            Ok(()) => ToolResult { success: true, output: "moved".into(), data: serde_json::json!(null) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn proc_launch(args: &serde_json::Value) -> ToolResult {
        let cmd = args["cmd"].as_str().unwrap_or("");
        let cargs: Vec<&str> = args["args"].as_array()
            .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();
        match Command::new(cmd).args(&cargs).spawn() {
            Ok(child) => ToolResult { success: true, output: format!("PID: {}", child.id()), data: serde_json::json!({"pid": child.id()}) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn proc_kill(args: &serde_json::Value) -> ToolResult {
        let pid = args["pid"].as_u64().unwrap_or(0) as u32;
        let signal = args["signal"].as_str().unwrap_or("TERM");
        match Command::new("kill").args([format!("-{signal}"), format!("{pid}")]).output() {
            Ok(o) => ToolResult { success: o.status.success(), output: String::from_utf8_lossy(&o.stdout).into(), data: serde_json::json!(null) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn proc_list(_args: &serde_json::Value) -> ToolResult {
        match Command::new("ps").args(["-eo", "pid,comm"]).output() {
            Ok(o) => ToolResult { success: true, output: String::from_utf8_lossy(&o.stdout).into(), data: serde_json::json!(null) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn sys_info(_args: &serde_json::Value) -> ToolResult {
        let mut info = serde_json::json!({
            "os": "Forge OS",
            "hostname": hostname(),
            "arch": std::env::consts::ARCH,
            "cpus": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1),
        });
        ToolResult { success: true, output: "system info".into(), data: info }
    }

    fn sys_shutdown(_args: &serde_json::Value) -> ToolResult {
        let _ = Command::new("shutdown").args(["-h", "now"]).spawn();
        ToolResult { success: true, output: "shutting down...".into(), data: serde_json::json!(null) }
    }

    fn sys_reboot(_args: &serde_json::Value) -> ToolResult {
        let _ = Command::new("reboot").spawn();
        ToolResult { success: true, output: "rebooting...".into(), data: serde_json::json!(null) }
    }

    fn sys_volume(args: &serde_json::Value) -> ToolResult {
        let level = args["level"].as_u64().unwrap_or(50);
        let _ = Command::new("amixer").args(["set", "Master", &format!("{level}%")]).output();
        ToolResult { success: true, output: format!("volume: {level}%"), data: serde_json::json!({"level": level}) }
    }

    fn net_http(args: &serde_json::Value) -> ToolResult {
        let url = args["url"].as_str().unwrap_or("");
        match Command::new("curl").args(["-s", "-L", "--max-time", "10", url]).output() {
            Ok(o) => ToolResult { success: o.status.success(), output: String::from_utf8_lossy(&o.stdout).into(), data: serde_json::json!(null) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn net_download(args: &serde_json::Value) -> ToolResult {
        let url = args["url"].as_str().unwrap_or("");
        let dest = args["dest"].as_str().unwrap_or("/tmp/download");
        match Command::new("curl").args(["-L", "-o", dest, url]).output() {
            Ok(o) => ToolResult { success: o.status.success(), output: format!("saved to {dest}"), data: serde_json::json!(null) },
            Err(e) => ToolResult { success: false, output: e.to_string(), data: serde_json::json!(null) },
        }
    }

    fn web_search(args: &serde_json::Value) -> ToolResult {
        let query = args["query"].as_str().unwrap_or("");
        ToolResult { success: true, output: format!("Search: {query} (integrate with web search API)"), data: serde_json::json!(null) }
    }

    fn web_browse(args: &serde_json::Value) -> ToolResult {
        let url = args["url"].as_str().unwrap_or("");
        ToolResult { success: true, output: format!("Browse: {url} (integrate with headless browser)"), data: serde_json::json!(null) }
    }

    fn display_screenshot(args: &serde_json::Value) -> ToolResult {
        let path = args["path"].as_str().unwrap_or("/tmp/screenshot.png");
        let _ = Command::new("import").args(["-window", "root", path]).output();
        ToolResult { success: true, output: format!("screenshot saved to {path}"), data: serde_json::json!(null) }
    }
}

fn hostname() -> String {
    std::fs::read_to_string("/etc/hostname")
        .unwrap_or_else(|_| "forge".into())
        .trim()
        .to_string()
}

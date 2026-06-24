// USB device guard — authorize, monitor, and log USB device events
// Uses Linux /sys/bus/usb/devices/ and usb authorization interface
// Human-centric: no USB device connects without the user knowing

use std::collections::HashSet;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct UsbGuard {
    whitelist: HashSet<String>,   // vendor:product IDs
    known_devices: HashSet<String>,
    policy: UsbPolicy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UsbPolicy {
    Ask,       // Prompt user for new devices (default)
    Whitelist, // Only allow whitelist devices
    Open,      // Allow all (insecure, not recommended)
}

#[derive(Debug, Clone)]
pub struct UsbEvent {
    pub timestamp: u64,
    pub vendor_id: String,
    pub product_id: String,
    pub serial: String,
    pub action: UsbAction,
    pub authorized: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UsbAction { Attached, Detached }

impl UsbGuard {
    pub fn new(policy: UsbPolicy) -> Self {
        let mut whitelist = HashSet::new();
        // Always allow common HID devices (keyboard, mouse)
        whitelist.insert("046d:c000".into()); // Logitech
        whitelist.insert("046d:c001".into());
        whitelist.insert("046d:c002".into());
        whitelist.insert("046d:c003".into());
        whitelist.insert("046d:c004".into());
        whitelist.insert("046d:c005".into());
        Self { whitelist, known_devices: HashSet::new(), policy }
    }

    pub fn add_whitelist(&mut self, vid_pid: &str) {
        self.whitelist.insert(vid_pid.to_string());
    }

    pub fn remove_whitelist(&mut self, vid_pid: &str) {
        self.whitelist.remove(vid_pid);
    }

    pub fn load_whitelist(&mut self, path: &str) -> std::io::Result<()> {
        let data = fs::read_to_string(path)?;
        for line in data.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') {
                self.whitelist.insert(line.to_string());
            }
        }
        Ok(())
    }

    pub fn scan(&mut self) -> Vec<UsbEvent> {
        let mut events = Vec::new();
        let usb_root = Path::new("/sys/bus/usb/devices/");
        if !usb_root.exists() { return events; }

        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let mut current = HashSet::new();

        if let Ok(entries) = fs::read_dir(usb_root) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                // Skip non-device entries
                if name == "usb1" || name == "usb2" { continue; }

                let id = name.to_string();
                current.insert(id.clone());

                if !self.known_devices.contains(&id) {
                    // New device detected
                    let (vid, pid) = self.read_ids(&path);
                    let serial = self.read_attr(&path, "serial").unwrap_or_default();
                    let authorized = self.authorize_device(&path, &vid, &pid);

                    events.push(UsbEvent {
                        timestamp: now,
                        vendor_id: vid.clone(),
                        product_id: pid.clone(),
                        serial,
                        action: UsbAction::Attached,
                        authorized,
                    });

                    if authorized {
                        println!("[USB] Authorized: {}:{} {}", vid, pid,
                            self.read_attr(&path, "product").unwrap_or_default());
                    } else {
                        println!("[USB] BLOCKED: {}:{} — not in whitelist", vid, pid);
                    }
                }
            }
        }

        // Detect removals
        let removed: Vec<_> = self.known_devices.difference(&current).cloned().collect();
        for id in &removed {
            events.push(UsbEvent {
                timestamp: now,
                vendor_id: String::new(),
                product_id: String::new(),
                serial: String::new(),
                action: UsbAction::Detached,
                authorized: true,
            });
            println!("[USB] Removed: {}", id);
        }

        self.known_devices = current;
        events
    }

    fn read_ids(&self, dev_path: &Path) -> (String, String) {
        let id_vendor = self.read_attr(dev_path, "idVendor").unwrap_or_default();
        let id_product = self.read_attr(dev_path, "idProduct").unwrap_or_default();
        let vid = id_vendor.trim().to_lowercase();
        let pid = id_product.trim().to_lowercase();
        (vid, pid)
    }

    fn read_attr(&self, dev_path: &Path, attr: &str) -> Option<String> {
        let p = dev_path.join(attr);
        fs::read_to_string(&p).ok().map(|s| s.trim().to_string())
    }

    fn authorize_device(&self, dev_path: &Path, vid: &str, pid: &str) -> bool {
        let key = format!("{}:{}", vid, pid);

        let allowed = match self.policy {
            UsbPolicy::Open => true,
            UsbPolicy::Whitelist => self.whitelist.contains(&key),
            UsbPolicy::Ask => {
                // In Ask mode, whitelist devices auto-pass; others need confirmation
                // For now, in headless/ISO mode, fall back to whitelist behavior
                self.whitelist.contains(&key)
            }
        };

        // Write to authorization control file
        let auth_path = dev_path.join("authorized");
        if auth_path.exists() {
            let val = if allowed { b"1" } else { b"0" };
            let _ = fs::write(&auth_path, val);
        }

        allowed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_whitelist_has_devices() {
        let guard = UsbGuard::new(UsbPolicy::Whitelist);
        assert!(guard.whitelist.len() >= 5);
    }

    #[test]
    fn test_add_whitelist() {
        let mut guard = UsbGuard::new(UsbPolicy::Whitelist);
        guard.add_whitelist("1234:5678");
        assert!(guard.whitelist.contains("1234:5678"));
    }
}
// Full disk encryption via LUKS/cryptsetup
// Integrated into PID 1 boot sequence
// Human-centric: passphrase prompt at boot, TPM-backed key optional

use std::fs;
use std::path::Path;
use std::process::Command;

pub struct LuksManager {
    mapped_name: String,
    device: String,
    tpm_key: Option<String>,
}

#[derive(Debug)]
pub enum LuksStatus {
    Unlocked(String),  // mapper path
    Locked,
    NotEncrypted,
    Error(String),
}

impl LuksManager {
    pub fn new(device: &str, mapped_name: &str) -> Self {
        Self {
            mapped_name: mapped_name.to_string(),
            device: device.to_string(),
            tpm_key: None,
        }
    }

    pub fn with_tpm(mut self, key_path: &str) -> Self {
        self.tpm_key = Some(key_path.to_string());
        self
    }

    /// Check if device has LUKS header
    pub fn is_luks(&self) -> bool {
        let output = Command::new("cryptsetup")
            .args(["isLuks", &self.device])
            .output();

        match output {
            Ok(o) => o.status.success(),
            Err(_) => false,
        }
    }

    /// Check current LUKS status
    pub fn status(&self) -> LuksStatus {
        let mapper_path = format!("/dev/mapper/{}", self.mapped_name);
        if Path::new(&mapper_path).exists() {
            return LuksStatus::Unlocked(mapper_path);
        }

        if self.is_luks() {
            return LuksStatus::Locked;
        }

        LuksStatus::NotEncrypted
    }

    /// Unlock LUKS volume — tries TPM key first, then passphrase
    pub fn unlock(&self, passphrase: Option<&str>) -> Result<String, String> {
        let mapper_path = format!("/dev/mapper/{}", self.mapped_name);

        // Already unlocked
        if Path::new(&mapper_path).exists() {
            println!("[LUKS] {} already unlocked at {}", self.device, mapper_path);
            return Ok(mapper_path);
        }

        if !self.is_luks() {
            return Err(format!("{} is not a LUKS device", self.device));
        }

        // Try TPM key first
        if let Some(ref tpm_path) = self.tpm_key {
            if Path::new(tpm_path).exists() {
                let output = Command::new("cryptsetup")
                    .args(["luksOpen", "--key-file", tpm_path, &self.device, &self.mapped_name])
                    .output();

                if let Ok(o) = output {
                    if o.status.success() {
                        println!("[LUKS] Unlocked {} via TPM key", self.device);
                        return Ok(mapper_path);
                    }
                }
            }
        }

        // Try passphrase
        if let Some(pw) = passphrase {
            let mut child = Command::new("cryptsetup")
                .args(["luksOpen", &self.device, &self.mapped_name, "--key-file=-"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to spawn cryptsetup: {}", e))?;

            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                let _ = stdin.write_all(pw.as_bytes());
                let _ = stdin.write_all(b"\n");
            }

            let output = child.wait_with_output()
                .map_err(|e| format!("cryptsetup error: {}", e))?;

            if output.status.success() {
                println!("[LUKS] Unlocked {} via passphrase", self.device);
                return Ok(mapper_path);
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("LUKS unlock failed: {}", stderr.trim()));
        }

        Err("No passphrase or TPM key provided".into())
    }

    /// Lock (close) the LUKS volume
    pub fn lock(&self) -> Result<(), String> {
        let output = Command::new("cryptsetup")
            .args(["luksClose", &self.mapped_name])
            .output()
            .map_err(|e| format!("cryptsetup error: {}", e))?;

        if output.status.success() {
            println!("[LUKS] Locked {}", self.mapped_name);
            Ok(())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(format!("LUKS lock failed: {}", stderr.trim()))
        }
    }

    /// Check if cryptsetup is available
    pub fn available() -> bool {
        Command::new("cryptsetup").arg("--version").output().map(|o| o.status.success()).unwrap_or(false)
    }

    /// Read passphrase from console (or from boot config file)
    pub fn read_passphrase_from_file(path: &str) -> Option<String> {
        fs::read_to_string(path).ok().map(|s| s.trim().to_string())
    }
}

/// LUKS initialization helper — for first-time setup
pub fn luks_format(device: &str, passphrase: &str) -> Result<(), String> {
    let mut child = Command::new("cryptsetup")
        .args(["--batch-mode", "luksFormat", device, "--key-file=-"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn cryptsetup: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        use std::io::Write;
        let _ = stdin.write_all(passphrase.as_bytes());
        let _ = stdin.write_all(b"\n");
    }

    let output = child.wait_with_output()
        .map_err(|e| format!("cryptsetup error: {}", e))?;

    if output.status.success() {
        println!("[LUKS] Formatted {}", device);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("LUKS format failed: {}", stderr.trim()))
    }
}
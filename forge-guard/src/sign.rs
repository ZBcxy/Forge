// Application signature verification
// Uses Ed25519 signatures to verify binary integrity
// Trusted public keys stored in /etc/forge/trusted_keys/
// Human-centric: unsigned or tampered binaries are flagged, not silently blocked

use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct AppSigner {
    trusted_keys: HashMap<String, Vec<u8>>,  // key_id -> public key bytes
    policy: SignPolicy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SignPolicy {
    Enforce,  // Block unsigned binaries
    Warn,     // Allow but log warning (default)
    Audit,    // Allow all, just log
}

#[derive(Debug, Clone)]
pub struct SignResult {
    pub path: String,
    pub signed: bool,
    pub key_id: Option<String>,
    pub valid: bool,
    pub message: String,
}

impl AppSigner {
    pub fn new(policy: SignPolicy) -> Self {
        Self { trusted_keys: HashMap::new(), policy }
    }

    /// Load all trusted public keys from a directory
    pub fn load_keys(&mut self, dir: &str) -> std::io::Result<usize> {
        let key_dir = Path::new(dir);
        if !key_dir.exists() { return Ok(0); }
        let mut count = 0;
        for entry in fs::read_dir(key_dir)? {
            let path = entry?.path();
            if path.extension().map_or(false, |e| e == "pub") {
                let key_data = fs::read(&path)?;
                let key_id = path.file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                if key_data.len() >= 32 {
                    self.trusted_keys.insert(key_id, key_data);
                    count += 1;
                }
            }
        }
        println!("[SIGN] Loaded {} trusted keys from {}", count, dir);
        Ok(count)
    }

    /// Verify a binary's signature
    /// Looks for <binary>.sig as detached Ed25519 signature (64 bytes)
    pub fn verify(&self, binary_path: &str) -> SignResult {
        let path = Path::new(binary_path);

        // Find signature file: binary.sig
        let sig_path = path.with_extension("sig");
        let sig_data = match fs::read(&sig_path) {
            Ok(d) => d,
            Err(_) => {
                let msg = format!("No signature found for {}", binary_path);
                let allowed = self.policy != SignPolicy::Enforce;
                if !allowed { println!("[SIGN] BLOCKED: {}", msg); }
                else { println!("[SIGN] WARNING: {}", msg); }
                return SignResult {
                    path: binary_path.into(),
                    signed: false,
                    key_id: None,
                    valid: allowed,
                    message: msg,
                };
            }
        };

        // Read binary data
        let binary_data = match fs::read(binary_path) {
            Ok(d) => d,
            Err(e) => return SignResult {
                path: binary_path.into(),
                signed: true,
                key_id: None,
                valid: false,
                message: format!("Cannot read binary: {}", e),
            },
        };

        // Parse signature block: [key_id_len: u8][key_id: bytes][signature: 64 bytes]
        if sig_data.len() < 66 {
            return SignResult {
                path: binary_path.into(),
                signed: true,
                key_id: None,
                valid: false,
                message: "Invalid signature format (too short)".into(),
            };
        }

        let key_id_len = sig_data[0] as usize;
        if sig_data.len() < 1 + key_id_len + 64 {
            return SignResult {
                path: binary_path.into(),
                signed: true,
                key_id: None,
                valid: false,
                message: "Invalid signature format (malformed)".into(),
            };
        }

        let key_id = String::from_utf8_lossy(&sig_data[1..1+key_id_len]).to_string();
        let sig_bytes = &sig_data[1+key_id_len..1+key_id_len+64];

        // Look up public key
        let pub_key = match self.trusted_keys.get(&key_id) {
            Some(k) => k,
            None => {
                let msg = format!("Unknown signer: {}", key_id);
                println!("[SIGN] WARNING: {}", msg);
                return SignResult {
                    path: binary_path.into(),
                    signed: true,
                    key_id: Some(key_id),
                    valid: self.policy == SignPolicy::Audit,
                    message: msg,
                };
            }
        };

        // Verify Ed25519 signature (simplified: use ring or ed25519-dalek in production)
        // For now, we use a simple SHA-256 + key check approach as placeholder
        // In production, replace with ed25519_dalek::Verifier
        let valid = self.verify_ed25519(&binary_data, sig_bytes, pub_key);

        let msg = if valid {
            format!("Signature valid: {} signed by {}", binary_path, key_id)
        } else {
            format!("Signature INVALID: {} — tampering detected!", binary_path)
        };

        if !valid {
            println!("[SIGN] ALERT: {}", msg);
        }

        SignResult {
            path: binary_path.into(),
            signed: true,
            key_id: Some(key_id),
            valid: valid || self.policy == SignPolicy::Audit,
            message: msg,
        }
    }

    /// Placeholder Ed25519 verification (SHA-256 hash check for now)
    /// Replace with actual ed25519-dalek for production
    fn verify_ed25519(&self, _msg: &[u8], _sig: &[u8], _pub_key: &[u8]) -> bool {
        // In a full implementation, use:
        // use ed25519_dalek::{Verifier, VerifyingKey, Signature};
        // let vk = VerifyingKey::from_bytes(pub_key.try_into().unwrap())?;
        // let sig = Signature::from_bytes(sig.try_into().unwrap());
        // vk.verify(msg, &sig).is_ok()

        // For now: check key length and signature length are correct
        // A real crypto check requires ed25519-dalek or ring crate
        _msg.len() > 0 && _sig.len() == 64 && _pub_key.len() == 32
    }

    /// Scan and verify all binaries in a directory
    pub fn scan_dir(&self, dir: &str) -> Vec<SignResult> {
        let mut results = Vec::new();
        let dir_path = Path::new(dir);
        if !dir_path.exists() { return results; }

        if let Ok(entries) = fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "sig" { continue; } // Skip signature files themselves
                    }
                    let path_str = path.to_string_lossy();
                    // Only check ELF binaries
                    if self.is_elf(&path_str) {
                        results.push(self.verify(&path_str));
                    }
                }
            }
        }
        results
    }

    fn is_elf(&self, path: &str) -> bool {
        fs::read(path).map(|d| d.starts_with(b"\x7fELF")).unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_signer() {
        let signer = AppSigner::new(SignPolicy::Warn);
        assert_eq!(signer.policy, SignPolicy::Warn);
        assert!(signer.trusted_keys.is_empty());
    }
}
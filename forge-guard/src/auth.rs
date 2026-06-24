use sha2::{Sha256, Digest};
use std::time::{Duration, Instant};

const MAX_ATTEMPTS: u32 = 5;
const LOCKOUT_SECS: u64 = 300;

pub struct AuthState {
    attempts: u32,
    locked_until: Option<Instant>,
    hash: Vec<u8>,
    voiceprint: bool,
}

impl AuthState {
    pub fn new(passphrase: &str) -> Self {
        let mut h = Sha256::new();
        h.update(b"forge-os-salt");
        h.update(passphrase.as_bytes());
        Self { attempts: 0, locked_until: None, hash: h.finalize().to_vec(), voiceprint: false }
    }
    pub fn locked(&self) -> bool {
        self.locked_until.map_or(false, |u| Instant::now() < u)
    }
    pub fn verify(&mut self, input: &str) -> Result<bool, &str> {
        if self.locked() { return Err("Locked. Wait 5 min."); }
        let mut h = Sha256::new();
        h.update(b"forge-os-salt");
        h.update(input.as_bytes());
        if h.finalize().to_vec() == self.hash { self.attempts = 0; return Ok(true); }
        self.attempts += 1;
        if self.attempts >= MAX_ATTEMPTS {
            self.locked_until = Some(Instant::now() + Duration::from_secs(LOCKOUT_SECS));
            return Err("Locked. 5 min timeout.");
        }
        Ok(false)
    }
}

pub async fn run_auth_daemon() {
    loop { tokio::time::sleep(Duration::from_secs(60)).await; }
}
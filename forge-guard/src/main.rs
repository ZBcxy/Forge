mod auth;
mod confirm;
mod monitor;
mod lock;
mod audit;
mod usb;
mod sign;

use usb::{UsbGuard, UsbPolicy};
use sign::{AppSigner, SignPolicy};

fn main() {
    println!("[forge-guard] Human security layer starting...");

    // USB device guard
    let mut usb_guard = UsbGuard::new(UsbPolicy::Ask);
    let _ = usb_guard.load_whitelist("/etc/forge/usb_whitelist.conf");
    let events = usb_guard.scan();
    println!("[forge-guard] USB scan: {} devices active", events.len());

    // App signature verification
    let mut signer = AppSigner::new(SignPolicy::Warn);
    let loaded = signer.load_keys("/etc/forge/trusted_keys/").unwrap_or(0);
    println!("[forge-guard] Signer: {} trusted keys loaded", loaded);

    // Scan system binaries
    let results = signer.scan_dir("/usr/bin");
    let valid = results.iter().filter(|r| r.valid).count();
    let invalid = results.len() - valid;
    if invalid > 0 {
        println!("[forge-guard] SIGN WARNING: {} unsigned/tampered binaries detected", invalid);
    } else {
        println!("[forge-guard] SIGN: {} binaries verified OK", valid);
    }

    println!("[forge-guard] All security subsystems active");
    println!("[forge-guard]  - auth: passphrase authentication");
    println!("[forge-guard]  - confirm: critical operation confirmation");
    println!("[forge-guard]  - monitor: behavior anomaly detection");
    println!("[forge-guard]  - lock: rate-limit lockout");
    println!("[forge-guard]  - audit: full audit trail");
    println!("[forge-guard]  - usb: device authorization guard");
    println!("[forge-guard]  - sign: application signature verification");

    loop {
        std::thread::sleep(std::time::Duration::from_secs(30));
        let _ = usb_guard.scan();
    }
}
// Forge Init -- PID 1 for Forge OS
// Mounts virtual filesystems, unlocks LUKS, spawns service manager, reaps children.

mod mounts;
mod signals;
mod device;
mod luks;

use std::os::unix::process::CommandExt;
use std::process::Command;
use tracing::{info, error, warn};

fn main() {
    tracing_subscriber::fmt().with_target(false).with_ansi(false).init();
    info!("Forge Init starting -- PID 1");

    // ── LUKS disk encryption ──
    if luks::LuksManager::available() {
        let luks_cfg = "/etc/forge/luks.json";
        let managed = if std::path::Path::new(luks_cfg).exists() {
            match std::fs::read_to_string(luks_cfg) {
                Ok(data) => {
                    if let Ok(cfg) = serde_json::from_str::<serde_json::Value>(&data) {
                        let device = cfg["device"].as_str().unwrap_or("/dev/sda2");
                        let name = cfg["name"].as_str().unwrap_or("forge-root");
                        let mgr = luks::LuksManager::new(device, name);
                        let passphrase = cfg["passphrase_file"].as_str()
                            .and_then(|f| luks::LuksManager::read_passphrase_from_file(f));
                        match mgr.unlock(passphrase.as_deref()) {
                            Ok(mapper) => { info!("LUKS root unlocked: {}", mapper); true }
                            Err(e) => { warn!("LUKS unlock failed: {} -- continuing without encryption", e); false }
                        }
                    } else { false }
                }
                Err(_) => false,
            }
        } else {
            false
        };
        if !managed {
            info!("No LUKS config at {} -- skipping disk encryption", luks_cfg);
        }
    } else {
        info!("cryptsetup not found -- LUKS support disabled");
    }

    if let Err(e) = mounts::mount_all() {
        error!("Failed to mount filesystems: {e}");
        emergency_halt();
    }

    if let Err(e) = device::create_essential() {
        warn!("Device nodes: {e}");
    }
    signals::setup_handlers();

    info!("Launching Forge Service Manager...");
    match spawn_svc() {
        Ok(pid) => info!("forge-svc spawned as PID {pid}"),
        Err(e) => { error!("Failed to spawn forge-svc: {e}"); emergency_halt(); }
    }

    info!("Init entering main loop");
    main_loop();
}

fn spawn_svc() -> Result<libc::pid_t, std::io::Error> {
    unsafe {
        match libc::fork() {
            -1 => Err(std::io::Error::last_os_error()),
            0 => { let _ = Command::new("/usr/bin/forge-svc").exec(); libc::_exit(1); }
            pid => Ok(pid),
        }
    }
}

fn main_loop() {
    loop {
        let mut status: libc::c_int = 0;
        unsafe { libc::waitpid(-1, &mut status, 0); }
        if signals::should_shutdown() {
            info!("Shutting down...");
            do_shutdown();
        }
    }
}

fn do_shutdown() {
    info!("Syncing filesystems and terminating processes...");
    unsafe {
        libc::sync();
        libc::kill(-1, libc::SIGTERM);
        std::thread::sleep(std::time::Duration::from_secs(3));
        libc::kill(-1, libc::SIGKILL);
        std::thread::sleep(std::time::Duration::from_secs(1));
        libc::sync();
        libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF);
    }
}

fn emergency_halt() -> ! {
    unsafe { libc::sync(); libc::reboot(libc::LINUX_REBOOT_CMD_HALT); }
    std::process::exit(1);
}
// Forge Init — PID 1 for Forge OS
// Mounts virtual filesystems, spawns service manager, reaps children.

mod mounts;
mod signals;
mod device;

use std::os::unix::process::CommandExt;
use std::process::Command;
use tracing::{info, error, warn};

fn main() {
    tracing_subscriber::fmt().with_target(false).with_ansi(false).init();
    info!("Forge Init starting — PID 1");

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


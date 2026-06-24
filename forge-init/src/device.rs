use anyhow::{Context, Result};
use nix::sys::stat::Mode;

pub fn create_essential() -> Result<()> {
    let nodes: &[(&str, libc::mode_t, u32, u32)] = &[
        ("/dev/null",    libc::S_IFCHR, 1, 3),
        ("/dev/zero",    libc::S_IFCHR, 1, 5),
        ("/dev/random",  libc::S_IFCHR, 1, 8),
        ("/dev/urandom", libc::S_IFCHR, 1, 9),
        ("/dev/console", libc::S_IFCHR, 5, 1),
        ("/dev/tty",     libc::S_IFCHR, 5, 0),
    ];
    for (path, mode, maj, min) in nodes {
        let dev = nix::sys::stat::makedev(u64::from(*maj), u64::from(*min));
        nix::sys::stat::mknod(*path, nix::sys::stat::SFlag::from_bits_truncate(*mode),
            Mode::from_bits_truncate(mode | 0o666), dev)
            .with_context(|| format!("mknod {path}"))?;
    }
    for i in 0..8 {
        let tty_path = format!("/dev/tty{i}");
        nix::sys::stat::mknod(tty_path.as_str(), nix::sys::stat::SFlag::S_IFCHR,
            Mode::from_bits_truncate(libc::S_IFCHR | 0o666),
            nix::sys::stat::makedev(4, (i as u64))).ok();
    }
    std::os::unix::fs::symlink("/proc/self/fd/0", "/dev/stdin").ok();
    std::os::unix::fs::symlink("/proc/self/fd/1", "/dev/stdout").ok();
    std::os::unix::fs::symlink("/proc/self/fd/2", "/dev/stderr").ok();
    Ok(())
}


use anyhow::{Context, Result};
use nix::mount::{mount, MsFlags};
use tracing::info;

pub fn mount_all() -> Result<()> {
    let mounts: &[(&str, &str, &str, MsFlags)] = &[
        ("proc", "/proc", "proc", MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC | MsFlags::MS_NODEV),
        ("sysfs", "/sys", "sysfs", MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC | MsFlags::MS_NODEV),
        ("devtmpfs", "/dev", "devtmpfs", MsFlags::MS_NOSUID),
        ("devpts", "/dev/pts", "devpts", MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC),
        ("tmpfs", "/dev/shm", "tmpfs", MsFlags::MS_NOSUID | MsFlags::MS_NODEV),
        ("tmpfs", "/run", "tmpfs", MsFlags::MS_NOSUID | MsFlags::MS_NODEV),
        ("tmpfs", "/tmp", "tmpfs", MsFlags::MS_NOSUID | MsFlags::MS_NODEV),
    ];

    for (src, tgt, fstype, flags) in mounts {
        std::fs::create_dir_all(tgt).with_context(|| format!("mkdir {tgt}"))?;
        mount(Some(*src), *tgt, Some(*fstype), *flags, Some(""))
            .with_context(|| format!("mount {src} -> {tgt}"))?;
        info!("mounted {tgt}");
    }

    std::fs::create_dir_all("/run/lock").ok();
    std::fs::create_dir_all("/sys/fs/cgroup").ok();
    mount(Some("cgroup2"), "/sys/fs/cgroup", Some("cgroup2"),
          MsFlags::MS_NOSUID | MsFlags::MS_NOEXEC | MsFlags::MS_NODEV, Some("")).ok();

    Ok(())
}

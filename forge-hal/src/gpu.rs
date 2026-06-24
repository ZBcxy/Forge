use anyhow::Result;
use serde::Serialize;
use tracing::info;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum GpuBackend { Cuda, Vulkan, Metal, OpenCL, CpuOnly }

#[derive(Debug, Clone, Serialize)]
pub struct GpuInfo { pub name: String, pub backend: GpuBackend, pub vram_mb: u64, pub accel: bool }

pub fn detect_gpu() -> Result<GpuInfo> {
    let render = std::path::Path::new("/dev/dri/renderD128");
    if render.exists() {
        let vendor = std::fs::read_to_string("/sys/class/drm/card0/device/vendor").unwrap_or_default();
        let (name, vram) = match vendor.trim() {
            "0x8086" => ("Intel GPU", 0u64),
            "0x1002" => ("AMD GPU", 4096u64),
            "0x10de" => ("NVIDIA GPU", 8192u64),
            _ => ("Unknown GPU", 2048u64),
        };
        return Ok(GpuInfo { name: name.into(), backend: GpuBackend::Vulkan, vram_mb: vram, accel: true });
    }
    if std::path::Path::new("/dev/nvidia0").exists() {
        return Ok(GpuInfo { name: "NVIDIA CUDA".into(), backend: GpuBackend::Cuda, vram_mb: 8192, accel: true });
    }
    Ok(GpuInfo { name: "CPU only".into(), backend: GpuBackend::CpuOnly, vram_mb: 0, accel: false })
}

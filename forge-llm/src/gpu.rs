// GPU acceleration for Forge LLM
// Queries HAL for GPU capabilities and configures llama.cpp accordingly

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GpuBackend {
    Vulkan,
    Cuda,
    Metal,
    CpuOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuConfig {
    pub backend: GpuBackend,
    pub vram_mb: u64,
    pub gpu_layers: usize,
    pub enabled: bool,
}

impl GpuConfig {
    /// Detect GPU capabilities by querying the HAL via its Unix socket.
    pub fn detect() -> Self {
        // Query HAL socket for GPU info
        let socket_path = "/run/forge/hal.sock";
        if let Ok(data) = std::fs::read_to_string(socket_path) {
            if let Ok(info) = serde_json::from_str::<serde_json::Value>(&data) {
                let backend = match info["gpu"]["backend"].as_str() {
                    Some("vulkan") => GpuBackend::Vulkan,
                    Some("cuda") => GpuBackend::Cuda,
                    Some("metal") => GpuBackend::Metal,
                    _ => GpuBackend::CpuOnly,
                };
                let vram = info["gpu"]["vram_mb"].as_u64().unwrap_or(0);
                let accel = info["gpu"]["accel"].as_bool().unwrap_or(false);

                if accel {
                    // Auto-tune GPU layers: offload ~50% of layers for >4GB VRAM
                    let layers = if vram >= 8192 { 33 }
                        else if vram >= 4096 { 24 }
                        else if vram > 0 { 16 }
                        else { 0 };

                    return GpuConfig {
                        backend,
                        vram_mb: vram,
                        gpu_layers: layers,
                        enabled: true,
                    };
                }
            }
        }

        // Fallback: CPU-only
        GpuConfig {
            backend: GpuBackend::CpuOnly,
            vram_mb: 0,
            gpu_layers: 0,
            enabled: false,
        }
    }

    /// Generate llama.cpp CLI arguments for GPU acceleration.
    pub fn to_cli_args(&self) -> Vec<String> {
        if !self.enabled || self.gpu_layers == 0 {
            return vec![];
        }
        vec![
            "--gpu-layers".into(),
            self.gpu_layers.to_string(),
        ]
    }

    /// Check if Vulkan SDK libraries are available.
    pub fn has_vulkan_libs() -> bool {
        std::path::Path::new("/usr/lib/libvulkan.so").exists()
            || std::path::Path::new("/usr/lib/x86_64-linux-gnu/libvulkan.so").exists()
    }

    /// Check if CUDA libraries are available.
    pub fn has_cuda_libs() -> bool {
        std::path::Path::new("/usr/lib/libcuda.so").exists()
            || std::path::Path::new("/usr/local/cuda/lib64/libcuda.so").exists()
    }
}
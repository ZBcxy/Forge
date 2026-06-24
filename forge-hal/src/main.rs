mod audio;
mod camera;
mod gpu;
mod ipc;

use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).with_ansi(false).init();
    info!("Forge HAL starting...");

    let audio_devices = audio::detect_devices()?;
    info!("Audio: {} mic(s), {} speaker(s)", audio_devices.mics.len(), audio_devices.speakers.len());

    let cameras = camera::detect_cameras().await?;
    info!("Camera: {} device(s)", cameras.len());

    let gpu_info = gpu::detect_gpu()?;
    info!("GPU: {} ({:?})", gpu_info.name, gpu_info.backend);

    ipc::serve().await?;
    Ok(())
}

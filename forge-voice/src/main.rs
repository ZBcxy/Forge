mod wake;
mod stt;
mod tts;
mod pipeline;

use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_target(false).with_ansi(false).init();
    info!("Forge Voice pipeline starting...");

    let wake = wake::WakeWordDetector::new("forge")?;
    let stt = stt::SpeechToText::new("/opt/forge/models/whisper-ggml-base.bin")?;
    let tts = tts::TextToSpeech::new()?;

    info!("Wake word: \"{}\"", wake.word());
    info!("Voice pipeline ready. Listening...");

    pipeline::run(wake, stt, tts).await?;
    Ok(())
}

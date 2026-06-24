use anyhow::Result;
use tracing::info;
use crate::wake::WakeWordDetector;
use crate::stt::SpeechToText;
use crate::tts::TextToSpeech;

pub async fn run(wake: WakeWordDetector, stt: SpeechToText, tts: TextToSpeech) -> Result<()> {
    info!("Voice pipeline active");

    loop {
        info!("Listening for wake word: \"{}\"...", wake.word());
        wake.listen_blocking()?;
        info!("Wake word detected!");
        tts.synthesize_and_play("请说")?;

        // Capture audio via HAL (stub for now — uses cpal in production)
        let audio_samples = vec![0.0f32; 16000 * 3]; // 3 seconds
        let text = stt.transcribe(&audio_samples, 16000)?;
        info!("Recognized: \"{}\"", text);

        if text.trim().is_empty() { continue; }
        tts.synthesize_and_play("收到")?;
    }
}

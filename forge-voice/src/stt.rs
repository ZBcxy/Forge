use anyhow::Result;
use std::path::Path;
use std::process::Command;
use tracing::info;

pub struct SpeechToText {
    whisper_cli: String,
    model_path: String,
}

impl SpeechToText {
    pub fn new(model_path: &str) -> Result<Self> {
        let whisper_cli = find_whisper_cli();
        info!("Whisper CLI: {whisper_cli}, model: {model_path}");
        Ok(Self { whisper_cli, model_path: model_path.into() })
    }

    pub fn transcribe(&self, _samples: &[f32], _sample_rate: u32) -> Result<String> {
        // For now: use whisper CLI on a WAV file path
        // In production, pipe raw audio via stdin
        if !Path::new(&self.whisper_cli).exists() || !Path::new(&self.model_path).exists() {
            return Ok("[STT stub]".into());
        }

        // Write samples to temp WAV file, then transcribe
        let tmp_wav = "/tmp/forge_voice.wav";
        let output = Command::new(&self.whisper_cli)
            .args(["-m", &self.model_path, "-f", tmp_wav, "-l", "zh", "--no-timestamps"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
            .unwrap_or_default();

        Ok(output)
    }
}

fn find_whisper_cli() -> String {
    for path in &["/usr/bin/whisper-cli", "/usr/local/bin/whisper-cli", "/opt/forge/whisper.cpp/build/bin/whisper-cli"] {
        if std::path::Path::new(path).exists() {
            return path.to_string();
        }
    }
    "whisper-cli".to_string()
}

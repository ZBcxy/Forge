use anyhow::Result;
use std::io::Write;
use std::process::{Command, Stdio};
use tracing::info;

pub struct TextToSpeech {
    piper_bin: String,
    model_path: String,
}

impl TextToSpeech {
    pub fn new() -> Result<Self> {
        Ok(Self {
            piper_bin: "/usr/bin/piper".into(),
            model_path: "/opt/forge/models/zh_CN.onnx".into(),
        })
    }

    pub fn synthesize(&self, text: &str) -> Result<Vec<i16>> {
        if !std::path::Path::new(&self.piper_bin).exists() {
            info!("piper not installed — TTS stub");
            return Ok(vec![0i16; 16000]);
        }

        // Pipe text to piper, capture WAV output
        let mut child = Command::new(&self.piper_bin)
            .args(["--model", &self.model_path, "--output-raw", "-"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }

        let output = child.wait_with_output()?;
        let samples: Vec<i16> = output.stdout
            .chunks_exact(2)
            .map(|c| i16::from_le_bytes([c[0], c[1]]))
            .collect();

        Ok(samples)
    }

    pub fn synthesize_and_play(&self, text: &str) -> Result<()> {
        let samples = self.synthesize(text)?;
        info!("TTS: {} samples generated for: {text}", samples.len());
        Ok(())
    }
}

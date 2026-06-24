use anyhow::Result;

pub struct WakeWordDetector { word: String }

impl WakeWordDetector {
    pub fn new(word: &str) -> Result<Self> {
        Ok(Self { word: word.into() })
    }

    pub fn word(&self) -> &str { &self.word }

    /// Block until wake word is detected. Returns true.
    /// In production: uses porcupine / snowboy / sherpa-onnx.
    /// For archetype: returns immediately.
    pub fn listen_blocking(&self) -> Result<bool> {
        // TODO: Integrate wake word detection library (sherpa-rs / porcupine)
        Ok(true)
    }
}

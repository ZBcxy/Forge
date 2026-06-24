use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AudioDevice { pub name: String, pub channels: u16, pub sample_rate: u32, pub is_default: bool }

#[derive(Debug, Clone, Serialize)]
pub struct AudioTopology { pub mics: Vec<AudioDevice>, pub speakers: Vec<AudioDevice> }

pub fn detect_devices() -> Result<AudioTopology> {
    let host = cpal::default_host();
    let default_input = host.default_input_device();
    let default_output = host.default_output_device();
    let mut mics = Vec::new();
    let mut speakers = Vec::new();

    for device in host.devices()? {
        let name = device.name()?;

        let is_def_in = default_input.as_ref()
            .and_then(|d| d.name().ok())
            .as_deref() == Some(name.as_str());

        let is_def_out = default_output.as_ref()
            .and_then(|d| d.name().ok())
            .as_deref() == Some(name.as_str());

        if let Ok(cfgs) = device.supported_input_configs() {
            if let Some(c) = cfgs.into_iter().next() {
                mics.push(AudioDevice { name: name.clone(), channels: c.channels(), sample_rate: c.max_sample_rate().0, is_default: is_def_in });
            }
        }
        if let Ok(cfgs) = device.supported_output_configs() {
            if let Some(c) = cfgs.into_iter().next() {
                speakers.push(AudioDevice { name, channels: c.channels(), sample_rate: c.max_sample_rate().0, is_default: is_def_out });
            }
        }
    }
    Ok(AudioTopology { mics, speakers })
}

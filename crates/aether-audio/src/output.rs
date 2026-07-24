use aether_core::{AetherError, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use rtrb::Consumer;

pub struct AudioOutputDevice {
    _stream: Stream,
    sample_rate: u32,
    channels: u16,
}

impl AudioOutputDevice {
    pub fn new(mut consumer: Consumer<f32>) -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| AetherError::AudioEngine("No default audio output device found".into()))?;

        let config: StreamConfig = device
            .default_output_config()
            .map_err(|e| AetherError::AudioEngine(format!("Failed to get output config: {}", e)))?
            .into();

        let sample_rate = config.sample_rate.0;
        let channels = config.channels;

        let err_fn = |err| {
            tracing::error!("CPAL audio output stream error: {}", err);
        };

        let stream = device
            .build_output_stream(
                &config,
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        *sample = consumer.pop().unwrap_or(0.0);
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| AetherError::AudioEngine(format!("Failed to build audio stream: {}", e)))?;

        stream
            .play()
            .map_err(|e| AetherError::AudioEngine(format!("Failed to play audio stream: {}", e)))?;

        Ok(Self {
            _stream: stream,
            sample_rate,
            channels,
        })
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> u16 {
        self.channels
    }
}

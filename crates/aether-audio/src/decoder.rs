use aether_core::{AetherError, Result};
use std::fs::File;
use std::path::Path;

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct AudioDecoder {
    format_reader: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    sample_rate: u32,
    channels: usize,
}

impl AudioDecoder {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let file = File::open(path.as_ref())?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.as_ref().extension().and_then(|s| s.to_str()) {
            hint.with_extension(ext);
        }

        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();
        let decoder_opts = DecoderOptions::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .map_err(|e| AetherError::Decoder(format!("Probe error: {}", e)))?;

        let format_reader = probed.format;

        let track = format_reader
            .default_track()
            .ok_or_else(|| AetherError::Decoder("No default audio track found".into()))?;

        let track_id = track.id;
        let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
        let channels = track.codec_params.channels.map(|c| c.count()).unwrap_or(2);

        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decoder_opts)
            .map_err(|e| AetherError::Decoder(format!("Codec creation error: {}", e)))?;

        Ok(Self {
            format_reader,
            decoder,
            track_id,
            sample_rate,
            channels,
        })
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    pub fn decode_next(&mut self, output: &mut Vec<f32>) -> Result<bool> {
        loop {
            let packet = match self.format_reader.next_packet() {
                Ok(packet) => packet,
                Err(symphonia::core::errors::Error::IoError(err))
                    if err.kind() == std::io::ErrorKind::UnexpectedEof =>
                {
                    return Ok(false);
                }
                Err(symphonia::core::errors::Error::ResetRequired) => {
                    return Ok(false);
                }
                Err(e) => return Err(AetherError::Decoder(format!("Packet read error: {}", e))),
            };

            if packet.track_id() != self.track_id {
                continue;
            }

            match self.decoder.decode(&packet) {
                Ok(audio_buf) => {
                    Self::copy_samples_f32(&audio_buf, output);
                    return Ok(true);
                }
                Err(symphonia::core::errors::Error::DecodeError(e)) => {
                    tracing::warn!("Decode warning: {}", e);
                    continue;
                }
                Err(e) => return Err(AetherError::Decoder(format!("Decode error: {}", e))),
            }
        }
    }

    fn copy_samples_f32(decoded: &AudioBufferRef, output: &mut Vec<f32>) {
        match decoded {
            AudioBufferRef::F32(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * num_channels);
                for frame in 0..num_frames {
                    for ch in 0..num_channels {
                        output.push(buf.chan(ch)[frame]);
                    }
                }
            }
            AudioBufferRef::U8(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * num_channels);
                for frame in 0..num_frames {
                    for ch in 0..num_channels {
                        let sample = (buf.chan(ch)[frame] as f32 - 128.0) / 128.0;
                        output.push(sample);
                    }
                }
            }
            AudioBufferRef::U16(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * num_channels);
                for frame in 0..num_frames {
                    for ch in 0..num_channels {
                        let sample = (buf.chan(ch)[frame] as f32 - 32768.0) / 32768.0;
                        output.push(sample);
                    }
                }
            }
            AudioBufferRef::S16(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * num_channels);
                for frame in 0..num_frames {
                    for ch in 0..num_channels {
                        let sample = buf.chan(ch)[frame] as f32 / 32768.0;
                        output.push(sample);
                    }
                }
            }
            AudioBufferRef::S24(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * num_channels);
                for frame in 0..num_frames {
                    for ch in 0..num_channels {
                        let sample = buf.chan(ch)[frame].0 as f32 / 8388608.0;
                        output.push(sample);
                    }
                }
            }
            AudioBufferRef::S32(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * num_channels);
                for frame in 0..num_frames {
                    for ch in 0..num_channels {
                        let sample = buf.chan(ch)[frame] as f32 / 2147483648.0;
                        output.push(sample);
                    }
                }
            }
            _ => {}
        }
    }
}

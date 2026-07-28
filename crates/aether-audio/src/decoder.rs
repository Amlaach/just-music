use aether_core::{AetherError, Result};
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Duration;

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::formats::{FormatOptions, FormatReader, SeekMode, SeekTo};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::units::Time;

pub struct AudioDecoder {
    format_reader: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    sample_rate: u32,
    channels: usize,
    duration_ms: u64,
}

impl AudioDecoder {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let raw_path = path.as_ref();
        let path_str = raw_path
            .to_string_lossy()
            .trim()
            .trim_matches('"')
            .trim_matches('\'')
            .to_string();
        let clean_path = PathBuf::from(&path_str);

        if !clean_path.exists() {
            return Err(AetherError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "Audio file does not exist at path: '{}'",
                    clean_path.display()
                ),
            )));
        }

        let file = File::open(&clean_path).map_err(|e| {
            AetherError::Io(std::io::Error::new(
                e.kind(),
                format!("Failed to open file '{}': {}", clean_path.display(), e),
            ))
        })?;

        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let mut hint = Hint::new();
        if let Some(ext) = clean_path.extension().and_then(|s| s.to_str()) {
            let lower_ext = ext.to_lowercase();
            hint.with_extension(&lower_ext);

            match lower_ext.as_str() {
                "mp3" => {
                    hint.mime_type("audio/mp3");
                }
                "flac" => {
                    hint.mime_type("audio/flac");
                }
                "wav" => {
                    hint.mime_type("audio/wav");
                }
                "ogg" | "opus" => {
                    hint.mime_type("audio/ogg");
                }
                "m4a" | "aac" | "mp4" => {
                    hint.mime_type("audio/mp4");
                }
                "aiff" | "aif" => {
                    hint.mime_type("audio/aiff");
                }
                _ => {}
            }
        }

        let format_opts = FormatOptions {
            enable_gapless: true,
            ..Default::default()
        };
        let metadata_opts = MetadataOptions::default();
        let decoder_opts = DecoderOptions::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .map_err(|e| {
                AetherError::Decoder(format!(
                    "Failed to probe audio format for '{}': {}",
                    clean_path.display(),
                    e
                ))
            })?;

        let format_reader = probed.format;

        let track = format_reader.default_track().ok_or_else(|| {
            AetherError::Decoder(format!(
                "No audio track found in '{}'",
                clean_path.display()
            ))
        })?;

        let track_id = track.id;
        let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
        let channels = track.codec_params.channels.map(|c| c.count()).unwrap_or(2);

        let n_frames = track.codec_params.n_frames;
        let duration_ms = if let Some(frames) = n_frames {
            if sample_rate > 0 {
                (frames * 1000) / sample_rate as u64
            } else {
                0
            }
        } else {
            0
        };

        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &decoder_opts)
            .map_err(|e| {
                AetherError::Decoder(format!(
                    "Failed to create codec decoder for '{}': {}",
                    clean_path.display(),
                    e
                ))
            })?;

        Ok(Self {
            format_reader,
            decoder,
            track_id,
            sample_rate,
            channels,
            duration_ms,
        })
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channels(&self) -> usize {
        self.channels
    }

    pub fn duration_ms(&self) -> u64 {
        self.duration_ms
    }

    pub fn seek(&mut self, time: Duration) -> Result<u64> {
        let seek_to = SeekTo::Time {
            time: Time::new(time.as_secs(), time.subsec_nanos() as f64 / 1_000_000_000.0),
            track_id: Some(self.track_id),
        };

        match self.format_reader.seek(SeekMode::Accurate, seek_to) {
            Ok(seeked_to) => Ok(seeked_to.actual_ts),
            Err(e) => Err(AetherError::Decoder(format!("Seek error: {}", e))),
        }
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
                output.reserve(num_frames * if num_channels == 1 { 2 } else { num_channels });
                for frame in 0..num_frames {
                    if num_channels == 1 {
                        let sample = buf.chan(0)[frame];
                        output.push(sample);
                        output.push(sample);
                    } else {
                        for ch in 0..num_channels {
                            output.push(buf.chan(ch)[frame]);
                        }
                    }
                }
            }
            AudioBufferRef::U8(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * if num_channels == 1 { 2 } else { num_channels });
                for frame in 0..num_frames {
                    if num_channels == 1 {
                        let sample = (buf.chan(0)[frame] as f32 - 128.0) / 128.0;
                        output.push(sample);
                        output.push(sample);
                    } else {
                        for ch in 0..num_channels {
                            let sample = (buf.chan(ch)[frame] as f32 - 128.0) / 128.0;
                            output.push(sample);
                        }
                    }
                }
            }
            AudioBufferRef::U16(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * if num_channels == 1 { 2 } else { num_channels });
                for frame in 0..num_frames {
                    if num_channels == 1 {
                        let sample = (buf.chan(0)[frame] as f32 - 32768.0) / 32768.0;
                        output.push(sample);
                        output.push(sample);
                    } else {
                        for ch in 0..num_channels {
                            let sample = (buf.chan(ch)[frame] as f32 - 32768.0) / 32768.0;
                            output.push(sample);
                        }
                    }
                }
            }
            AudioBufferRef::S16(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * if num_channels == 1 { 2 } else { num_channels });
                for frame in 0..num_frames {
                    if num_channels == 1 {
                        let sample = buf.chan(0)[frame] as f32 / 32768.0;
                        output.push(sample);
                        output.push(sample);
                    } else {
                        for ch in 0..num_channels {
                            let sample = buf.chan(ch)[frame] as f32 / 32768.0;
                            output.push(sample);
                        }
                    }
                }
            }
            AudioBufferRef::S24(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * if num_channels == 1 { 2 } else { num_channels });
                for frame in 0..num_frames {
                    if num_channels == 1 {
                        let sample = buf.chan(0)[frame].0 as f32 / 8388608.0;
                        output.push(sample);
                        output.push(sample);
                    } else {
                        for ch in 0..num_channels {
                            let sample = buf.chan(ch)[frame].0 as f32 / 8388608.0;
                            output.push(sample);
                        }
                    }
                }
            }
            AudioBufferRef::S32(buf) => {
                let num_channels = buf.spec().channels.count();
                let num_frames = buf.frames();
                output.reserve(num_frames * if num_channels == 1 { 2 } else { num_channels });
                for frame in 0..num_frames {
                    if num_channels == 1 {
                        let sample = buf.chan(0)[frame] as f32 / 2147483648.0;
                        output.push(sample);
                        output.push(sample);
                    } else {
                        for ch in 0..num_channels {
                            let sample = buf.chan(ch)[frame] as f32 / 2147483648.0;
                            output.push(sample);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_open_non_existent_file_returns_error() {
        let res = AudioDecoder::open("non_existent_audio_file_12345.mp3");
        assert!(res.is_err());
        if let Err(err) = res {
            let err_msg = err.to_string();
            assert!(err_msg.contains("Audio file does not exist"));
        }
    }

    #[test]
    fn test_decoder_open_quoted_path_cleaning() {
        let res = AudioDecoder::open("\"non_existent_file_quoted.wav\"");
        assert!(res.is_err());
        if let Err(err) = res {
            let err_msg = err.to_string();
            assert!(err_msg.contains("non_existent_file_quoted.wav"));
            assert!(!err_msg.contains("\"non_existent_file_quoted.wav\""));
        }
    }
}

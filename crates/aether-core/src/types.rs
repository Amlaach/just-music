use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Unique identifier for audio tracks.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TrackId(pub String);

impl TrackId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().to_string())
    }
}

impl Default for TrackId {
    fn default() -> Self {
        Self::new()
    }
}

/// Supported high-fidelity audio formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioFormat {
    Mp3,
    Flac,
    Wav,
    Aac,
    Ogg,
    Opus,
    Aiff,
    Alac,
    Unknown,
}

impl AudioFormat {
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "mp3" => AudioFormat::Mp3,
            "flac" => AudioFormat::Flac,
            "wav" => AudioFormat::Wav,
            "aac" | "m4a" => AudioFormat::Aac,
            "ogg" => AudioFormat::Ogg,
            "opus" => AudioFormat::Opus,
            "aiff" | "aif" => AudioFormat::Aiff,
            "alac" => AudioFormat::Alac,
            _ => AudioFormat::Unknown,
        }
    }
}

/// Core domain metadata representing a music track.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: TrackId,
    pub file_path: PathBuf,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub track_number: Option<u32>,
    pub duration_ms: u64,
    pub bitrate: Option<u32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub format: AudioFormat,
    pub replaygain_track_gain: Option<f32>,
    pub replaygain_track_peak: Option<f32>,
    pub play_count: u32,
    pub rating: u8,
}

/// Current playback state of the audio engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayState {
    Stopped,
    Playing,
    Paused,
}

/// ReplayGain mode setting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayGainMode {
    Off,
    Track,
    Album,
}

/// Current playback position info.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaybackPosition {
    pub current_ms: u64,
    pub total_ms: u64,
    pub progress_ratio: f32, // 0.0 to 1.0
}

use crate::types::{ReplayGainMode, TrackId};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Metadata update payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataUpdate {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub rating: Option<u8>,
}

/// Commands emitted by UI, CLI, or API clients to mutate state or control playback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerCommand {
    LoadTrack(PathBuf),
    Play,
    Pause,
    TogglePlayPause,
    Stop,
    SeekTo(Duration),
    NextTrack,
    PreviousTrack,
    SetVolume(f32), // 0.0 to 1.0
    SetMute(bool),
    EnqueueTrack(TrackId),
    RemoveFromQueue(usize),
    ClearQueue,
    SetEqualizerBand { band_index: usize, gain_db: f32 },
    SetEqualizerEnabled(bool),
    SetReplayGainMode(ReplayGainMode),
    UpdateTrackMetadata { track_id: TrackId, metadata: MetadataUpdate },
    SetRating { track_id: TrackId, rating: u8 },
}

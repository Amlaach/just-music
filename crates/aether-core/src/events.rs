use crate::types::{PlayState, PlaybackPosition, Track};
use serde::{Deserialize, Serialize};

/// Domain events emitted by the Core / Engine to notify subscribers (UI, Diagnostic Monitors, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerEvent {
    StateChanged(PlayState),
    TrackStarted(Track),
    TrackEnded,
    PositionUpdated(PlaybackPosition),
    VolumeChanged(f32),
    MuteChanged(bool),
    QueueUpdated(Vec<Track>),
    ErrorOccurred(String),
}

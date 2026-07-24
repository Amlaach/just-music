use thiserror::Error;

/// Core domain and system error type for Aether Sound System.
#[derive(Debug, Error)]
pub enum AetherError {
    #[error("Audio engine error: {0}")]
    AudioEngine(String),

    #[error("Decoder error: {0}")]
    Decoder(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Track not found: {0}")]
    TrackNotFound(String),

    #[error("Invalid audio format: {0}")]
    InvalidFormat(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Plugin error: {0}")]
    Plugin(String),

    #[error("Operation cancelled")]
    Cancelled,
}

pub type Result<T> = std::result::Result<T, AetherError>;

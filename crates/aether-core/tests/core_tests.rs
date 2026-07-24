use aether_core::*;
use std::time::Duration;

#[test]
fn test_audio_format_resolution() {
    assert_eq!(AudioFormat::from_extension("mp3"), AudioFormat::Mp3);
    assert_eq!(AudioFormat::from_extension("FLAC"), AudioFormat::Flac);
    assert_eq!(AudioFormat::from_extension("wav"), AudioFormat::Wav);
    assert_eq!(AudioFormat::from_extension("m4a"), AudioFormat::Aac);
    assert_eq!(AudioFormat::from_extension("unknown"), AudioFormat::Unknown);
}

#[test]
fn test_track_id_uniqueness() {
    let id1 = TrackId::new();
    let id2 = TrackId::new();
    assert_ne!(id1, id2);
}

#[test]
fn test_command_serialization() {
    let cmd = PlayerCommand::SeekTo(Duration::from_secs(45));
    let json = serde_json::to_string(&cmd).unwrap();
    assert!(json.contains("SeekTo"));
}

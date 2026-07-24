use aether_core::{AudioFormat, Track, TrackId};
use aether_storage::{Database, InstantSearchEngine};
use std::path::PathBuf;

#[test]
fn test_database_and_tantivy_instant_search() {
    let db = Database::in_memory().unwrap();
    let search = InstantSearchEngine::new_in_memory().unwrap();

    let track = Track {
        id: TrackId::new(),
        file_path: PathBuf::from("C:/Music/test_hebrew.flac"),
        title: "שיר חדש בעברית".into(),
        artist: "אמן מפורסם".into(),
        album: "אלבום בכורה".into(),
        genre: Some("Pop".into()),
        year: Some(2024),
        track_number: Some(1),
        duration_ms: 210000,
        bitrate: Some(1411),
        sample_rate: 44100,
        channels: 2,
        format: AudioFormat::Flac,
        replaygain_track_gain: None,
        replaygain_track_peak: None,
        play_count: 0,
        rating: 5,
    };

    db.insert_or_update_track(&track).unwrap();
    search.index_track(&track).unwrap();

    let fetched = db.get_track_by_id(&track.id).unwrap().unwrap();
    assert_eq!(fetched.title, "שיר חדש בעברית");

    let search_res = search.search("שיר", 10).unwrap();
    assert_eq!(search_res.len(), 1);
    assert_eq!(search_res[0], track.id);
}

use aether_core::{AetherError, Result};
use rusqlite::Connection;

pub fn initialize_schema(conn: &Connection) -> Result<()> {
    // Enable PRAGMA WAL for high-performance concurrent reads and writes
    conn.execute_batch(
        "
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = NORMAL;
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS tracks (
            id TEXT PRIMARY KEY NOT NULL,
            file_path TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            album TEXT NOT NULL,
            genre TEXT,
            year INTEGER,
            track_number INTEGER,
            duration_ms INTEGER NOT NULL,
            bitrate INTEGER,
            sample_rate INTEGER NOT NULL,
            channels INTEGER NOT NULL,
            format TEXT NOT NULL,
            replaygain_track_gain REAL,
            replaygain_track_peak REAL,
            play_count INTEGER DEFAULT 0,
            rating INTEGER DEFAULT 0,
            added_at TEXT NOT NULL,
            modified_at TEXT NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_tracks_artist_album ON tracks(artist, album);
        CREATE INDEX IF NOT EXISTS idx_tracks_title ON tracks(title);

        CREATE TABLE IF NOT EXISTS playlists (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            is_smart BOOLEAN DEFAULT 0,
            rules_json TEXT,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS playlist_tracks (
            playlist_id TEXT NOT NULL,
            track_id TEXT NOT NULL,
            position INTEGER NOT NULL,
            PRIMARY KEY (playlist_id, track_id),
            FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
            FOREIGN KEY (track_id) REFERENCES tracks(id) ON DELETE CASCADE
        );
        ",
    )
    .map_err(|e| AetherError::Storage(format!("Failed to initialize schema: {}", e)))?;

    Ok(())
}

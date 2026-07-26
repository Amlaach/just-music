use crate::schema::initialize_schema;
use aether_core::{AetherError, AudioFormat, Result, Track, TrackId};
use rusqlite::{params, Connection};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)
            .map_err(|e| AetherError::Storage(format!("Database open error: {}", e)))?;

        initialize_schema(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| AetherError::Storage(format!("Memory DB error: {}", e)))?;

        initialize_schema(&conn)?;

        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn execute_fts_init(&self) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AetherError::Storage("Mutex lock error".into()))?;

        conn.execute_batch(
            "CREATE VIRTUAL TABLE IF NOT EXISTS tracks_fts USING fts5(
                id UNINDEXED,
                title,
                artist,
                album,
                genre,
                tokenize = 'unicode61'
            );",
        )
        .map_err(|e| AetherError::Storage(format!("FTS5 init error: {}", e)))?;

        Ok(())
    }

    pub fn index_track_fts(&self, track: &Track) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AetherError::Storage("Mutex lock error".into()))?;

        let genre_str = track.genre.as_deref().unwrap_or("");

        conn.execute(
            "INSERT OR REPLACE INTO tracks_fts (id, title, artist, album, genre) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![track.id.0, track.title, track.artist, track.album, genre_str],
        )
        .map_err(|e| AetherError::Storage(format!("FTS5 index error: {}", e)))?;

        Ok(())
    }

    pub fn search_fts(&self, query_str: &str, limit: usize) -> Result<Vec<TrackId>> {
        let clean_query = query_str.trim();
        if clean_query.is_empty() {
            return Ok(Vec::new());
        }

        let conn = self
            .conn
            .lock()
            .map_err(|_| AetherError::Storage("Mutex lock error".into()))?;

        let formatted_query = format!("{}*", clean_query);

        let mut stmt = conn
            .prepare("SELECT id FROM tracks_fts WHERE tracks_fts MATCH ?1 LIMIT ?2")
            .map_err(|e| AetherError::Storage(format!("FTS5 prepare query error: {}", e)))?;

        let rows = stmt
            .query_map(params![formatted_query, limit as i64], |row| {
                let id_str: String = row.get(0)?;
                Ok(TrackId(id_str))
            })
            .map_err(|e| AetherError::Storage(format!("FTS5 search error: {}", e)))?;

        let mut track_ids = Vec::new();
        for r in rows {
            if let Ok(id) = r {
                track_ids.push(id);
            }
        }

        Ok(track_ids)
    }

    pub fn insert_or_update_track(&self, track: &Track) -> Result<()> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AetherError::Storage("Mutex lock error".into()))?;

        let format_str = format!("{:?}", track.format);
        let added_at = chrono::Utc::now().to_rfc3339();
        let modified_at = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT INTO tracks (
                id, file_path, title, artist, album, genre, year, track_number,
                duration_ms, bitrate, sample_rate, channels, format,
                replaygain_track_gain, replaygain_track_peak, play_count, rating,
                added_at, modified_at
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19
            ) ON CONFLICT(file_path) DO UPDATE SET
                title=excluded.title, artist=excluded.artist, album=excluded.album,
                genre=excluded.genre, year=excluded.year, track_number=excluded.track_number,
                duration_ms=excluded.duration_ms, bitrate=excluded.bitrate,
                sample_rate=excluded.sample_rate, channels=excluded.channels,
                format=excluded.format, modified_at=excluded.modified_at",
            params![
                track.id.0,
                track.file_path.to_string_lossy(),
                track.title,
                track.artist,
                track.album,
                track.genre,
                track.year,
                track.track_number,
                track.duration_ms,
                track.bitrate,
                track.sample_rate,
                track.channels,
                format_str,
                track.replaygain_track_gain,
                track.replaygain_track_peak,
                track.play_count,
                track.rating,
                added_at,
                modified_at,
            ],
        )
        .map_err(|e| AetherError::Storage(format!("Track insert error: {}", e)))?;

        Ok(())
    }

    pub fn get_track_by_id(&self, id: &TrackId) -> Result<Option<Track>> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AetherError::Storage("Mutex lock error".into()))?;

        let mut stmt = conn
            .prepare("SELECT id, file_path, title, artist, album, genre, year, track_number, duration_ms, bitrate, sample_rate, channels, format, replaygain_track_gain, replaygain_track_peak, play_count, rating FROM tracks WHERE id = ?1")
            .map_err(|e| AetherError::Storage(e.to_string()))?;

        let mut rows = stmt
            .query(params![id.0])
            .map_err(|e| AetherError::Storage(e.to_string()))?;

        if let Some(row) = rows
            .next()
            .map_err(|e| AetherError::Storage(e.to_string()))?
        {
            let path_str: String = row.get(1).unwrap_or_default();
            let format_str: String = row.get(12).unwrap_or_default();

            let track = Track {
                id: TrackId(row.get(0).unwrap_or_default()),
                file_path: PathBuf::from(path_str),
                title: row.get(2).unwrap_or_default(),
                artist: row.get(3).unwrap_or_default(),
                album: row.get(4).unwrap_or_default(),
                genre: row.get(5).ok(),
                year: row.get(6).ok(),
                track_number: row.get(7).ok(),
                duration_ms: row.get(8).unwrap_or_default(),
                bitrate: row.get(9).ok(),
                sample_rate: row.get(10).unwrap_or(44100),
                channels: row.get(11).unwrap_or(2),
                format: AudioFormat::from_extension(&format_str),
                replaygain_track_gain: row.get(13).ok(),
                replaygain_track_peak: row.get(14).ok(),
                play_count: row.get(15).unwrap_or(0),
                rating: row.get(16).unwrap_or(0),
            };
            Ok(Some(track))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>> {
        let conn = self
            .conn
            .lock()
            .map_err(|_| AetherError::Storage("Mutex lock error".into()))?;

        let mut stmt = conn
            .prepare("SELECT id, file_path, title, artist, album, genre, year, track_number, duration_ms, bitrate, sample_rate, channels, format, replaygain_track_gain, replaygain_track_peak, play_count, rating FROM tracks ORDER BY artist, album, track_number")
            .map_err(|e| AetherError::Storage(e.to_string()))?;

        let track_iter = stmt
            .query_map([], |row| {
                let path_str: String = row.get(1)?;
                let format_str: String = row.get(12)?;
                Ok(Track {
                    id: TrackId(row.get(0)?),
                    file_path: PathBuf::from(path_str),
                    title: row.get(2)?,
                    artist: row.get(3)?,
                    album: row.get(4)?,
                    genre: row.get(5).ok(),
                    year: row.get(6).ok(),
                    track_number: row.get(7).ok(),
                    duration_ms: row.get(8)?,
                    bitrate: row.get(9).ok(),
                    sample_rate: row.get(10)?,
                    channels: row.get(11)?,
                    format: AudioFormat::from_extension(&format_str),
                    replaygain_track_gain: row.get(13).ok(),
                    replaygain_track_peak: row.get(14).ok(),
                    play_count: row.get(15)?,
                    rating: row.get(16)?,
                })
            })
            .map_err(|e| AetherError::Storage(e.to_string()))?;

        let mut tracks = Vec::new();
        for t in track_iter {
            if let Ok(track) = t {
                tracks.push(track);
            }
        }
        Ok(tracks)
    }
}

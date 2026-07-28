use aether_core::{AetherError, Result, Track, TrackId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

#[derive(Debug, Default, Serialize, Deserialize)]
struct StorageData {
    tracks: HashMap<String, Track>,
}

#[derive(Clone)]
pub struct Database {
    db_path: Option<PathBuf>,
    data: Arc<RwLock<StorageData>>,
    fts_index: Arc<RwLock<HashMap<String, Vec<TrackId>>>>,
}

impl Database {
    pub fn open<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let path = db_path.as_ref().to_path_buf();
        let data = if path.exists() {
            let content = fs::read_to_string(&path)
                .map_err(|e| AetherError::Storage(format!("Failed to read database: {}", e)))?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            StorageData::default()
        };

        let db = Self {
            db_path: Some(path),
            data: Arc::new(RwLock::new(data)),
            fts_index: Arc::new(RwLock::new(HashMap::new())),
        };

        // Populate initial FTS index
        db.rebuild_fts_index()?;

        Ok(db)
    }

    pub fn in_memory() -> Result<Self> {
        Ok(Self {
            db_path: None,
            data: Arc::new(RwLock::new(StorageData::default())),
            fts_index: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    fn persist(&self) -> Result<()> {
        if let Some(path) = &self.db_path {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let data = self
                .data
                .read()
                .map_err(|_| AetherError::Storage("Lock error".into()))?;
            let json = serde_json::to_string_pretty(&*data)
                .map_err(|e| AetherError::Storage(format!("Serialize error: {}", e)))?;
            fs::write(path, json)
                .map_err(|e| AetherError::Storage(format!("Write error: {}", e)))?;
        }
        Ok(())
    }

    fn rebuild_fts_index(&self) -> Result<()> {
        let data = self
            .data
            .read()
            .map_err(|_| AetherError::Storage("Lock error".into()))?;
        for track in data.tracks.values() {
            self.index_track_fts(track)?;
        }
        Ok(())
    }

    pub fn execute_fts_init(&self) -> Result<()> {
        Ok(())
    }

    pub fn index_track_fts(&self, track: &Track) -> Result<()> {
        let mut index = self
            .fts_index
            .write()
            .map_err(|_| AetherError::Storage("Lock error".into()))?;

        let search_text = format!(
            "{} {} {} {}",
            track.title,
            track.artist,
            track.album,
            track.genre.as_deref().unwrap_or("")
        )
        .to_lowercase();

        index.insert(search_text, vec![track.id.clone()]);
        Ok(())
    }

    pub fn search_fts(&self, query_str: &str, limit: usize) -> Result<Vec<TrackId>> {
        let clean_query = query_str.trim().to_lowercase();
        if clean_query.is_empty() {
            return Ok(Vec::new());
        }

        let data = self
            .data
            .read()
            .map_err(|_| AetherError::Storage("Lock error".into()))?;

        let mut results = Vec::new();
        for track in data.tracks.values() {
            let title_match = track.title.to_lowercase().contains(&clean_query);
            let artist_match = track.artist.to_lowercase().contains(&clean_query);
            let album_match = track.album.to_lowercase().contains(&clean_query);
            let genre_match = track
                .genre
                .as_ref()
                .map(|g| g.to_lowercase().contains(&clean_query))
                .unwrap_or(false);

            if title_match || artist_match || album_match || genre_match {
                results.push(track.id.clone());
                if results.len() >= limit {
                    break;
                }
            }
        }

        Ok(results)
    }

    pub fn insert_or_update_track(&self, track: &Track) -> Result<()> {
        {
            let mut data = self
                .data
                .write()
                .map_err(|_| AetherError::Storage("Lock error".into()))?;
            data.tracks.insert(track.id.0.clone(), track.clone());
        }

        self.index_track_fts(track)?;
        self.persist()?;
        Ok(())
    }

    pub fn get_track_by_id(&self, id: &TrackId) -> Result<Option<Track>> {
        let data = self
            .data
            .read()
            .map_err(|_| AetherError::Storage("Lock error".into()))?;
        Ok(data.tracks.get(&id.0).cloned())
    }

    pub fn get_all_tracks(&self) -> Result<Vec<Track>> {
        let data = self
            .data
            .read()
            .map_err(|_| AetherError::Storage("Lock error".into()))?;
        let mut tracks: Vec<Track> = data.tracks.values().cloned().collect();
        tracks.sort_by(|a, b| match a.artist.cmp(&b.artist) {
            std::cmp::Ordering::Equal => a.album.cmp(&b.album),
            other => other,
        });
        Ok(tracks)
    }
}

use crate::db::Database;
use aether_core::{AetherError, Result, Track, TrackId};
use rusqlite::params;

#[derive(Clone)]
pub struct InstantSearchEngine {
    db: Database,
}

impl InstantSearchEngine {
    pub fn new(db: Database) -> Result<Self> {
        // Initialize FTS5 virtual table in SQLite
        db.execute_fts_init()?;
        Ok(Self { db })
    }

    pub fn index_track(&self, track: &Track) -> Result<()> {
        self.db.index_track_fts(track)
    }

    pub fn search(&self, query_str: &str, limit: usize) -> Result<Vec<TrackId>> {
        self.db.search_fts(query_str, limit)
    }
}

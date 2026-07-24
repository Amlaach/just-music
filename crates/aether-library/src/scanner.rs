use crate::metadata::MetadataExtractor;
use aether_core::Result;
use aether_storage::{Database, InstantSearchEngine};
use std::path::Path;
use walkdir::WalkDir;

pub struct LibraryScanner {
    db: Database,
    search_engine: InstantSearchEngine,
}

impl LibraryScanner {
    pub fn new(db: Database, search_engine: InstantSearchEngine) -> Self {
        Self { db, search_engine }
    }

    pub fn scan_directory<P: AsRef<Path>>(&self, dir_path: P) -> Result<usize> {
        let mut count = 0;

        for entry in WalkDir::new(dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if matches!(
                    ext_lower.as_str(),
                    "mp3" | "flac" | "wav" | "aac" | "m4a" | "ogg" | "opus" | "aiff"
                ) {
                    if let Ok(track) = MetadataExtractor::extract(path) {
                        if self.db.insert_or_update_track(&track).is_ok() {
                            let _ = self.search_engine.index_track(&track);
                            count += 1;
                        }
                    }
                }
            }
        }

        tracing::info!("Scanned and indexed {} audio tracks", count);
        Ok(count)
    }
}

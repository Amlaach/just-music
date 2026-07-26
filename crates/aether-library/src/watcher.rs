use crate::scanner::LibraryScanner;
use aether_core::{AetherError, Result};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::thread;

pub struct LibraryWatcher {
    _watcher: RecommendedWatcher,
    watch_path: PathBuf,
}

impl LibraryWatcher {
    pub fn watch<P: AsRef<Path>>(path: P, scanner: LibraryScanner) -> Result<Self> {
        let watch_path = path.as_ref().to_path_buf();
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(tx, Config::default())
            .map_err(|e| AetherError::Storage(format!("Watcher creation error: {}", e)))?;

        watcher
            .watch(&watch_path, RecursiveMode::Recursive)
            .map_err(|e| AetherError::Storage(format!("Watch path error: {}", e)))?;

        let path_clone = watch_path.clone();
        thread::spawn(move || {
            while let Ok(res) = rx.recv() {
                match res {
                    Ok(Event {
                        kind: EventKind::Create(_) | EventKind::Modify(_),
                        paths,
                        ..
                    }) => {
                        for p in paths {
                            if p.is_file() {
                                if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                                    if matches!(
                                        ext.to_lowercase().as_str(),
                                        "mp3"
                                            | "flac"
                                            | "wav"
                                            | "aac"
                                            | "m4a"
                                            | "ogg"
                                            | "opus"
                                            | "aiff"
                                    ) {
                                        tracing::info!(
                                            "Real-time library change detected: {:?}",
                                            p
                                        );
                                        let _ = scanner.scan_directory(&path_clone);
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        Ok(Self {
            _watcher: watcher,
            watch_path,
        })
    }
}

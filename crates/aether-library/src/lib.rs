pub mod metadata;
pub mod scanner;
pub mod watcher;

pub use metadata::MetadataExtractor;
pub use scanner::LibraryScanner;
pub use watcher::LibraryWatcher;

pub fn library_init() {
    tracing::info!("Aether Library Background Scanner & File Watcher ready");
}

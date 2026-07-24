use crate::disk::DiskCache;
use crate::memory::MemoryLruCache;
use aether_core::Result;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct CacheManager {
    memory_artwork: Arc<MemoryLruCache<String, Vec<u8>>>,
    disk_cache: Arc<DiskCache>,
}

impl CacheManager {
    pub fn new<P: AsRef<Path>>(disk_cache_dir: P) -> Result<Self> {
        let disk = DiskCache::new(disk_cache_dir)?;
        Ok(Self {
            memory_artwork: Arc::new(MemoryLruCache::new(200)), // 200 items in RAM
            disk_cache: Arc::new(disk),
        })
    }

    pub fn get_artwork(&self, track_id: &str) -> Option<Vec<u8>> {
        // 1. Check in-memory LRU
        if let Some(bytes) = self.memory_artwork.get(&track_id.to_string()) {
            return Some(bytes);
        }

        // 2. Check disk cache
        if let Some(bytes) = self.disk_cache.get_bytes(track_id) {
            self.memory_artwork.put(track_id.to_string(), bytes.clone());
            return Some(bytes);
        }

        None
    }

    pub fn store_artwork(&self, track_id: &str, data: &[u8]) -> Result<()> {
        self.memory_artwork.put(track_id.to_string(), data.to_vec());
        self.disk_cache.put_bytes(track_id, data)?;
        Ok(())
    }
}

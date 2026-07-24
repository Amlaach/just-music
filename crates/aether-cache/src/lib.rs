pub mod disk;
pub mod manager;
pub mod memory;

pub use disk::DiskCache;
pub use manager::CacheManager;
pub use memory::MemoryLruCache;

pub fn cache_init() {
    tracing::info!("Aether Multi-Tier Cache Engine ready");
}

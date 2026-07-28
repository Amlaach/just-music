use lru::LruCache;
use std::num::NonZeroUsize;
use std::sync::Mutex;

pub struct MemoryLruCache<K, V> {
    cache: Mutex<LruCache<K, V>>,
}

impl<K: std::hash::Hash + Eq, V: Clone> MemoryLruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        let cap = NonZeroUsize::new(capacity).unwrap_or(NonZeroUsize::MIN);
        Self {
            cache: Mutex::new(LruCache::new(cap)),
        }
    }

    pub fn get(&self, key: &K) -> Option<V> {
        let mut cache = self.cache.lock().ok()?;
        cache.get(key).cloned()
    }

    pub fn put(&self, key: K, value: V) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.put(key, value);
        }
    }

    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }
}

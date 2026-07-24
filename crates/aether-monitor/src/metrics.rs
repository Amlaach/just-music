use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricSnapshot {
    pub cpu_usage_pct: f32,
    pub memory_rss_mb: f32,
    pub fps: f32,
    pub audio_buffer_fill_pct: f32,
    pub decoder_latency_ms: f32,
    pub db_query_latency_ms: f32,
    pub cache_hit_ratio: f32,
}

#[derive(Clone)]
pub struct MonitorEngine {
    db_latency_ns: Arc<AtomicU64>,
    cache_hits: Arc<AtomicU64>,
    cache_misses: Arc<AtomicU64>,
}

impl MonitorEngine {
    pub fn new() -> Self {
        Self {
            db_latency_ns: Arc::new(AtomicU64::new(0)),
            cache_hits: Arc::new(AtomicU64::new(0)),
            cache_misses: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn record_db_latency(&self, duration_ns: u64) {
        self.db_latency_ns.store(duration_ns, Ordering::Relaxed);
    }

    pub fn record_cache_hit(&self) {
        self.cache_hits.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_cache_miss(&self) {
        self.cache_misses.fetch_add(1, Ordering::Relaxed);
    }

    pub fn snapshot(&self) -> MetricSnapshot {
        let hits = self.cache_hits.load(Ordering::Relaxed) as f32;
        let misses = self.cache_misses.load(Ordering::Relaxed) as f32;
        let total = hits + misses;
        let ratio = if total > 0.0 { hits / total } else { 1.0 };

        MetricSnapshot {
            cpu_usage_pct: 0.5,
            memory_rss_mb: 28.4,
            fps: 120.0,
            audio_buffer_fill_pct: 85.0,
            decoder_latency_ms: 0.12,
            db_query_latency_ms: self.db_latency_ns.load(Ordering::Relaxed) as f32 / 1_000_000.0,
            cache_hit_ratio: ratio,
        }
    }
}

impl Default for MonitorEngine {
    fn default() -> Self {
        Self::new()
    }
}

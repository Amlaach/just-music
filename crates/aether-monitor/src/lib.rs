pub mod metrics;

pub use metrics::{MetricSnapshot, MonitorEngine};

pub fn monitor_init() {
    tracing::info!("Aether Internal Diagnostics & Monitoring Engine initialized");
}

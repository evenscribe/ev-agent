pub mod logger;
pub mod metrics;
pub use logger::LogsCollector;
pub use metrics::MetricsCollector;

use async_trait::async_trait;

#[async_trait]
pub trait Collector: Send + Sync {
    async fn transmit(&mut self);
}

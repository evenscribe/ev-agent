pub mod logger;
pub mod metrics;

pub use logger::LogsCollector;
pub use metrics::MetricsCollector;

pub enum Collector {
    Metrics(MetricsCollector),
    Logs(LogsCollector),
}

impl Collector {
    pub fn transmit(self) {
        tokio::spawn(async move {
            match self {
                Collector::Metrics(mut collector) => collector.transmit().await,
                Collector::Logs(mut collector) => collector.transmit().await,
            }
        });
    }
}

mod logs;
mod profiler;
mod tracer;

use crate::event::Event;
pub use logs::LogsIntegration;
pub use profiler::ProfilerIntegration;
use std::sync::Arc;
use tokio::sync::mpsc;
pub use tracer::TracerIntegration;
use yaml_rust2::Yaml;

#[derive(Debug)]
pub enum Integration {
    Logs(LogsIntegration),
    Trace(TracerIntegration),
    Profiler(ProfilerIntegration),
}

impl Integration {
    pub async fn new(
        integration: &Yaml,
        service_transmitter: mpsc::Sender<Event>,
        service_name: &str,
    ) -> Self {
        let integration = integration.as_hash().expect("Integration should be a hash");
        let (key, value) = integration
            .iter()
            .next()
            .expect("Integration should have a single key-value pair");
        let name = key.as_str().expect("Integration name should be a string");
        match name {
            "logs" => Integration::Logs(
                LogsIntegration::new(&value, service_transmitter, service_name).await,
            ),
            "trace" => Integration::Trace(TracerIntegration),
            "profiler" => Integration::Profiler(ProfilerIntegration),
            _ => panic!("Unknown integration type: {}", name),
        }
    }

    pub async fn run(&mut self) {
        match self {
            Integration::Logs(logs) => logs.run().await,
            Integration::Trace(_) => todo!(),
            Integration::Profiler(_) => todo!(),
        }
    }
}

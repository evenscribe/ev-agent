mod logs;
mod profiler;
mod tracer;

pub use logs::LogsIntegration;
pub use profiler::ProfilerIntegration;
pub use tracer::TracerIntegration;
use yaml_rust2::Yaml;

#[derive(Debug)]
pub enum Integration {
    Logs(LogsIntegration),
    Trace(TracerIntegration),
    Profiler(ProfilerIntegration),
}

impl Integration {
    pub fn new(integration: &Yaml) -> Self {
        let integration = integration.as_hash().expect("Integration should be a hash");
        let (key, value) = integration
            .iter()
            .next()
            .expect("Integration should have a single key-value pair");
        let name = key.as_str().expect("Integration name should be a string");
        match name {
            "logs" => Integration::Logs(LogsIntegration::new(&value)),
            "trace" => Integration::Trace(TracerIntegration),
            "profiler" => Integration::Profiler(ProfilerIntegration),
            _ => panic!("Unknown integration type: {}", name),
        }
    }
}

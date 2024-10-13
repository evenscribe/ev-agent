#[derive(Debug)]
pub enum EventType {
    Logs,
    Metrics,
}

#[derive(Debug)]
pub struct Event {
    pub(crate) event_type: EventType,
    pub(crate) details: String, // Just a JSON BLOB for simplicity... {our open telemetry JSON or mean, sd, variance},
}

impl Event {
    pub fn new_log(details: String) -> Self {
        Self {
            event_type: EventType::Logs,
            details,
        }
    }
    pub fn new_metric(details: &str) -> Self {
        Self {
            event_type: EventType::Metrics,
            details: details.to_string(),
        }
    }
}

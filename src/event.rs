#[derive(Debug)]
pub struct Event {
    service_name: String,
    integration_name: String,
    data: String,
    timestamp: i64,
}

impl Event {
    pub fn new(
        service_name: String,
        integration_name: String,
        data: String,
        timestamp: i64,
    ) -> Event {
        Event {
            service_name,
            integration_name,
            data,
            timestamp,
        }
    }
}

use super::{tailers::Tailer, LogsCollector};

pub struct LogsCollectorBuilder {
    tailers: Vec<Tailer>,
}

impl LogsCollectorBuilder {
    pub fn new() -> Self {
        Self {
            tailers: Vec::new(),
        }
    }

    pub fn with_tailer(mut self, tailer: Tailer) -> Self {
        self.tailers.push(tailer);
        self
    }

    pub fn build(self) -> Result<LogsCollector, String> {
        if self.tailers.is_empty() {
            return Err("No tailers provided".to_string());
        }
        Ok(LogsCollector {
            tailers: self.tailers,
        })
    }
}

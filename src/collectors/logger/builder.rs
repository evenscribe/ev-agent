use tokio::sync::mpsc;

use super::{tailers::Tailable, LogsCollector};
use crate::event::Event;

pub struct LogsCollectorBuilder {
    tx: Option<mpsc::Sender<Event>>,
    tailers: Vec<Box<dyn Tailable>>,
}

impl LogsCollectorBuilder {
    pub fn new() -> Self {
        Self {
            tx: None,
            tailers: Vec::new(),
        }
    }

    pub fn with_tx(mut self, tx: mpsc::Sender<Event>) -> Self {
        self.tx = Some(tx);
        self
    }

    pub fn with_tailer(mut self, tailer: Box<dyn Tailable>) -> Self {
        self.tailers.push(tailer);
        self
    }

    pub fn build(self) -> Result<LogsCollector, String> {
        if self.tx.is_none() {
            return Err("No tx provided".to_string());
        }
        if self.tailers.is_empty() {
            return Err("No tailers provided".to_string());
        }
        Ok(LogsCollector {
            tx: self.tx.unwrap(),
            tailers: self.tailers,
        })
    }
}

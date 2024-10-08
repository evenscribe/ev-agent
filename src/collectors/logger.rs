use super::Collector;
use crate::event::Event;
use async_trait::async_trait;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct LogsCollector {
    tx: mpsc::Sender<Event>,
}

impl LogsCollector {
    pub fn new(tx: mpsc::Sender<Event>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl Collector for LogsCollector {
    async fn transmit(&mut self) {
        loop {
            let ev = Event::new_log("This is a Log JSON BLOB....");
            if let Err(_) = self.tx.send(ev).await {
                println!("receiver dropped");
                return;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

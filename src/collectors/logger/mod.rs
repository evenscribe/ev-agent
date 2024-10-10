mod builder;
mod tailers;

use super::Collector;
use crate::event::Event;
use async_trait::async_trait;
use tailers::Tailable;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct LogsCollector {
    tx: mpsc::Sender<Event>,
    tailers: Vec<Box<dyn Tailable>>,
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

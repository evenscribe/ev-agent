use crate::event::Event;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Clone)]
pub struct MetricsCollector {
    tx: mpsc::Sender<Arc<Event>>,
}

impl MetricsCollector {
    pub fn new(tx: mpsc::Sender<Arc<Event>>) -> Self {
        Self { tx }
    }

    pub async fn transmit(&mut self) {
        loop {
            let ev = Event::new_metric("This is a Metric JSON BLOB....");
            if let Err(_) = self.tx.send(Arc::new(ev)).await {
                println!("receiver dropped");
                return;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    }
}

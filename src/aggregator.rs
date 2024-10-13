use crate::event::Event;
use std::sync::Arc;
use tokio::sync::mpsc;

pub(crate) struct Aggregator {
    rx: mpsc::Receiver<Arc<Event>>,
}

impl Aggregator {
    pub fn new(rx: mpsc::Receiver<Arc<Event>>) -> Self {
        Self { rx }
    }

    pub async fn run(&mut self) {
        while let Some(i) = self.rx.recv().await {
            println!("{:?}", i);
        }
    }
}

use tokio::sync::mpsc;

use crate::event::Event;

pub(crate) struct Aggregator {
    rx: mpsc::Receiver<Event>,
}

impl Aggregator {
    pub fn new(rx: mpsc::Receiver<Event>) -> Self {
        Self { rx }
    }

    pub async fn run(&mut self) {
        while let Some(i) = self.rx.recv().await {
            println!("{:?}", i);
        }
    }
}

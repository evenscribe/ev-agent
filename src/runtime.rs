use crate::{aggregator::Aggregator, collectors::Collector, event::Event};
use std::sync::Arc;
use tokio::sync::mpsc;

pub(crate) struct Runtime {
    aggregator: Aggregator,
    collectors: Vec<Collector>,
    transmitter: mpsc::Sender<Arc<Event>>,
}

impl Runtime {
    pub(crate) fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Arc<Event>>(100);
        let aggregator = Aggregator::new(rx);
        let collectors = vec![];
        Self {
            aggregator,
            collectors,
            transmitter: tx,
        }
    }

    pub(crate) async fn run(self) {
        let mut aggregator = self.aggregator;
        let collectors = self.collectors;
        tokio::spawn(async move { aggregator.run().await });
        for collector in collectors {
            tokio::spawn(async move { collector.transmit() });
        }
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    }

    pub(crate) fn get_transmitter(&self) -> mpsc::Sender<Arc<Event>> {
        self.transmitter.clone()
    }

    pub(crate) fn add_collector(&mut self, collector: Collector) {
        self.collectors.push(collector);
    }
}

use crate::{aggregator::Aggregator, collectors::Collector, event::Event};
use tokio::sync::mpsc;

pub(crate) struct Runtime {
    aggregator: Aggregator,
    // TODO: Change Box to Arc
    collectors: Vec<Box<dyn Collector>>,
    transmitter: mpsc::Sender<Event>,
}

impl Runtime {
    pub(crate) fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Event>(100);
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
        for mut collector in collectors {
            tokio::spawn(async move { collector.transmit().await });
        }
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    }

    pub(crate) fn get_transmitter(&self) -> mpsc::Sender<Event> {
        self.transmitter.clone()
    }

    pub(crate) fn add_collector(&mut self, collector: Box<dyn Collector>) {
        self.collectors.push(collector);
    }
}

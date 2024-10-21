use std::sync::Arc;

use crate::{event::Event, integration::Integration};
use tokio::sync::{mpsc, Mutex};
use yaml_rust2::Yaml;

#[derive(Debug)]
pub struct Service {
    pub name: String,
    pub integrations: Vec<Arc<Mutex<Integration>>>,
    agent_transmitter: mpsc::Sender<Arc<Event>>,
    receiver: mpsc::Receiver<Arc<Event>>,
}

impl Service {
    pub async fn new(service: &Yaml, agent_transmitter: mpsc::Sender<Arc<Event>>) -> Self {
        let (transmitter, receiver) = mpsc::channel::<Arc<Event>>(1000);
        let service = service.as_hash().expect("Service should be a hash");
        let (key, value) = service
            .iter()
            .next()
            .expect("Service should have a single key-value pair");
        let name = key.as_str().expect("Service name should be a string");

        let mut integrations = vec![];

        let integrations_yaml = value
            .as_vec().
            expect(&format!("Integrations array under {name} has not be formatted properly in config.\nHint: It should be an array of objects"));

        for integration in integrations_yaml {
            integrations.push(Arc::new(Mutex::new(
                Integration::new(&integration, transmitter.clone(), name).await,
            )))
        }

        Self {
            name: name.to_string(),
            integrations,
            agent_transmitter,
            receiver,
        }
    }

    pub async fn run(&mut self) {
        for integration in &self.integrations {
            let i = Arc::clone(integration);
            tokio::spawn(async move {
                i.lock().await.run().await;
            });
        }

        self.aggregate().await;
    }

    pub async fn transmit(&self, event: Arc<Event>) {
        self.agent_transmitter.send(event).await.unwrap();
    }

    pub async fn aggregate(&mut self) {
        loop {
            let event = self.receiver.recv().await.unwrap();
            self.transmit(event).await;
        }
    }
}

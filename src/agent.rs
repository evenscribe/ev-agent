use std::sync::Arc;
use tokio::sync::mpsc;
use yaml_rust2::YamlLoader;

use crate::{event::Event, service::Service};

#[derive(Debug)]
pub struct Agent {
    pub name: String,
    pub api_key: String,
    pub monitor: bool,
    pub services: Vec<Service>,
    receiver: mpsc::Receiver<Arc<Event>>,
}

impl Agent {
    pub async fn new() -> Self {
        println!("[evagent] Intializing agent");
        let (transmitter, receiver) = mpsc::channel::<Arc<Event>>(1000);

        let path = std::env::var("HOME").expect("Could not find home directory");
        let path = format!("{path}/ev.yaml");
        let config = match std::fs::read_to_string(path) {
            Ok(file) => file,
            Err(e) => panic!("Could not read config file.\nDetails: {:?}", e),
        };
        let config = match YamlLoader::load_from_str(&config) {
            Ok(file) => file,
            Err(e) => panic!("Could not parse config file.\nDetails: {:?}", e),
        };
        let config = &config[0];

        let name = config["name"]
            .as_str()
            .expect("Could not find a required field `name` in config");

        let api_key = config["api_key"]
            .as_str()
            .expect("Could not find a required field `api_key` in config");

        let monitor = match config["monitor"].as_bool() {
            Some(monitor) => monitor,
            None => true,
        };

        let mut services = vec![];

        let services_yaml = config["services"]
            .as_vec().
            expect("`services` field has not be formatted properly in config.\nHint: It should be an array of objects");

        for service in services_yaml {
            services.push(Service::new(&service, transmitter.clone()).await)
        }

        Self {
            name: name.to_string(),
            api_key: api_key.to_string(),
            monitor,
            services,
            receiver,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let services = std::mem::take(&mut self.services);
        for mut service in services.into_iter() {
            tokio::spawn(async move {
                service.run().await;
            });
        }

        self.aggregate().await;
        Ok(())
    }

    pub async fn aggregate(&mut self) {
        loop {
            let event = self.receiver.recv().await.unwrap();
            println!("{:?}", event);
        }
    }
}

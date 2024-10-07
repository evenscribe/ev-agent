use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

#[derive(Clone)]
struct Service {
    tx: mpsc::Sender<u32>,
}

#[derive(Clone)]
struct Aggregator {
    rx: Arc<Mutex<mpsc::Receiver<u32>>>,
}

impl Service {
    async fn run(&self) {
        let mut i = 0;
        loop {
            if let Err(_) = self.tx.send(i).await {
                println!("receiver dropped");
                return;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            i += 1
        }
    }
}

impl Aggregator {
    async fn run(&self) {
        let mut rx = self.rx.lock().await;
        while let Some(i) = rx.recv().await {
            println!("got = {}", i);
        }
    }
}

struct Rt {
    aggregator: Aggregator,
    service: Service,
}

impl Rt {
    pub fn new() -> Rt {
        let (tx, rx) = mpsc::channel(100);
        let aggregator = Aggregator {
            rx: Arc::new(Mutex::new(rx)),
        };
        let service = Service { tx };
        Rt {
            aggregator,
            service,
        }
    }

    pub async fn run(&self) {
        let aggregator = self.aggregator.clone();
        tokio::spawn(async move {
            aggregator.run().await;
        });
        let service = self.service.clone();
        tokio::spawn(async move {
            service.run().await;
        });
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60 * 60)).await;
        }
    }
}

#[tokio::main]
async fn main() {
    let rt = Rt::new();
    rt.run().await;
}


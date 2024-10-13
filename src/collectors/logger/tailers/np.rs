use std::sync::Arc;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::mpsc::Sender;
use tokio::time;

use crate::event::Event;

#[derive(Debug)]
pub struct PipeTailer {
    pipe_path: String,
    tx: Sender<Arc<Event>>,
}

impl PipeTailer {
    pub fn new(pipe_path: String, tx: Sender<Arc<Event>>) -> Self {
        Self { pipe_path, tx }
    }

    async fn open_pipe(&self) -> File {
        File::open(&self.pipe_path)
            .await
            .expect("Failed to open named pipe")
    }

    pub async fn tail(&mut self) {
        println!("Tailing named pipe: {}", self.pipe_path);

        let file = self.open_pipe().await;
        let mut reader = BufReader::new(file).lines();

        loop {
            match reader.next_line().await {
                Ok(Some(line)) => {
                    let ev = Event::new_log(line);
                    self.tx
                        .send(Arc::new(ev))
                        .await
                        .expect("Failed to send logs from np");
                }
                Ok(None) => {
                    time::sleep(Duration::from_millis(1000)).await;
                }
                Err(e) => {
                    eprintln!("Error reading from pipe: {}", e);
                    break;
                }
            }
        }
    }
}

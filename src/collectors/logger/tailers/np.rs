use async_trait::async_trait;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time;

use super::Tailable;

struct PipeTailer {
    pipe_path: String,
}

impl PipeTailer {
    pub fn new(pipe_path: String) -> Self {
        Self { pipe_path }
    }

    async fn open_pipe(&self) -> File {
        File::open(&self.pipe_path)
            .await
            .expect("Failed to open named pipe")
    }
}

#[async_trait]
impl Tailable for PipeTailer {
    async fn tail(&mut self) {
        println!("Tailing named pipe: {}", self.pipe_path);

        let file = self.open_pipe().await;
        let mut reader = BufReader::new(file).lines();

        loop {
            match reader.next_line().await {
                Ok(Some(line)) => {
                    println!("{}", line);
                }
                Ok(None) => {
                    time::sleep(Duration::from_millis(500)).await;
                }
                Err(e) => {
                    eprintln!("Error reading from pipe: {}", e);
                    break;
                }
            }
        }
    }
}

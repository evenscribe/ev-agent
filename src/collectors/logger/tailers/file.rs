use crate::event::Event;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};
use tokio::sync::mpsc::Sender;
use tokio::time::{sleep, Duration};

#[derive(Debug)]
pub struct FileTailer {
    file_name: String,
    reader: BufReader<File>,
    position: u64,
    tx: Sender<Arc<Event>>,
}

impl FileTailer {
    pub async fn new(file_name: String, tx: Sender<Arc<Event>>) -> Self {
        let file = File::open(&file_name).await.expect("Failed to open file");
        let mut reader = BufReader::new(file);
        let position = reader
            .seek(SeekFrom::End(0))
            .await
            .expect("Failed to seek to end of file");

        Self {
            file_name,
            reader,
            position,
            tx,
        }
    }

    async fn file_changed(&self) -> bool {
        let metadata = tokio::fs::metadata(&self.file_name)
            .await
            .expect("Failed to get file metadata");
        metadata.len() < self.position
    }

    async fn reopen(&mut self) {
        let file = File::open(&self.file_name)
            .await
            .expect("Failed to reopen file");
        self.reader = BufReader::new(file);
        self.position = 0;
    }

    pub async fn tail(&mut self) {
        println!("Tailing file: {}", self.file_name);

        loop {
            if self.file_changed().await {
                println!("File was truncated or rotated, reopening...");
                self.reopen().await;
            }

            let mut line = String::new();

            match self.reader.read_line(&mut line).await {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        print!("{}", line);
                        self.position += bytes_read as u64;
                    } else {
                        sleep(Duration::from_millis(500)).await;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading file: {}", e);
                    break;
                }
            }
        }
    }
}

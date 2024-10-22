use crate::event::Event;
use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

use super::Tailer;

#[derive(Debug)]
pub struct FileTailer {
    reader: BufReader<File>,
    position: u64,
    transmiter: mpsc::Sender<Event>,
    owner: String, // Name of the service that owns this Integration -> Tailer
    path: String,
}

impl FileTailer {
    pub async fn new(
        path: String,
        owner: String, // Name of the service that owns this Integration
        seek_from: SeekFrom,
        transmiter: mpsc::Sender<Event>,
    ) -> Self {
        let file = File::open(&path).await.expect("Failed to open file");
        let mut reader = BufReader::new(file);
        let position = reader
            .seek(seek_from)
            .await
            .expect("Failed to seek to end of file");

        Self {
            reader,
            position,
            transmiter,
            owner,
            path,
        }
    }

    async fn file_changed(&self) -> bool {
        let metadata = tokio::fs::metadata(&self.path)
            .await
            .expect("Failed to get file metadata");
        metadata.len() < self.position
    }

    async fn reopen(&mut self) {
        let file = File::open(&self.path).await.expect("Failed to reopen file");
        self.reader = BufReader::new(file);
        self.position = 0;
    }
}

#[async_trait]
impl Tailer for FileTailer {
    async fn tail(&mut self) {
        println!("[evagent] Tailing file: {}", self.path);

        loop {
            if self.file_changed().await {
                println!("[evagent] File was truncated or rotated, reopening...");
                self.reopen().await;
            }

            let mut line = String::new();

            match self.reader.read_line(&mut line).await {
                Ok(bytes_read) => {
                    if bytes_read > 0 {
                        self.transmiter
                            .send(Event::new(
                                self.owner.clone(),
                                "Logs.FileTailer".into(),
                                line.trim().into(),
                                1111222,
                            ))
                            .await
                            .expect("Failed to send event");
                        self.position += bytes_read as u64;
                    } else {
                        sleep(Duration::from_millis(500)).await;
                    }
                }
                Err(e) => {
                    eprintln!("[evagent] Error reading file: {}", e);
                    break;
                }
            }
        }
    }
}

use async_trait::async_trait;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};
use tokio::time::{sleep, Duration};

use super::Tailable;

struct FileTailer {
    file_name: String,
    reader: BufReader<File>,
    position: u64,
}

impl FileTailer {
    pub async fn new(file_name: String) -> Self {
        let file = File::open(&file_name).await.expect("Failed to open file");
        let reader = BufReader::new(file);

        Self {
            file_name,
            reader,
            position: 0,
        }
    }

    pub async fn initialize(&mut self) {
        self.position = self
            .reader
            .seek(SeekFrom::End(0))
            .await
            .expect("Failed to seek to end of file");
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
}

#[async_trait]
impl Tailable for FileTailer {
    async fn tail(&mut self) {
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

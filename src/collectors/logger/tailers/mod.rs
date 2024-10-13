mod darwin_log;
mod file;
mod journalctl;
mod np;

use crate::event::Event;
pub use file::FileTailer;
pub use np::PipeTailer;
use std::sync::Arc;
use tokio::sync::mpsc::Sender;

pub enum Tailer {
    File(FileTailer),
    DarwinLog,
    JournalCtl,
    NP(PipeTailer),
}

impl Tailer {
    pub async fn new_file(file_name: String, tx: Sender<Arc<Event>>) -> Self {
        Self::File(FileTailer::new(file_name, tx).await)
    }

    pub fn new_np(pipe_path: String, tx: Sender<Arc<Event>>) -> Self {
        Self::NP(PipeTailer::new(pipe_path, tx))
    }

    pub fn tail(self) {
        tokio::spawn(async move {
            match self {
                Tailer::File(mut t) => t.tail().await,
                Tailer::DarwinLog => unimplemented!(),
                Tailer::JournalCtl => unimplemented!(),
                Tailer::NP(mut t) => t.tail().await,
            }
        });
    }
}

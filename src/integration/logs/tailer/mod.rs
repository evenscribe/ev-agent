mod file;
use async_trait::async_trait;
pub use file::FileTailer;
use std::fmt::Debug;

#[async_trait]
pub trait Tailer: Debug + Send + Sync {
    async fn tail(&mut self);
}

use async_trait::async_trait;

mod darwin_log;
mod file;
mod journalctl;
mod np;

#[async_trait]
pub trait Tailable: Send + Sync {
    async fn tail(&mut self);
}

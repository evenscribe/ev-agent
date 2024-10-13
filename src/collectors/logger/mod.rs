mod builder;
mod tailers;

pub use builder::LogsCollectorBuilder;
pub use tailers::Tailer;

pub struct LogsCollector {
    tailers: Vec<Tailer>,
}

impl LogsCollector {
    pub fn new(tailers: Vec<Tailer>) -> Self {
        Self { tailers }
    }

    pub async fn transmit(&mut self) {
        for _ in 0..self.tailers.len() {
            let tailer = self.tailers.pop().unwrap();
            tailer.tail();
        }
    }
}

mod aggregator;
mod collectors;
mod event;
mod runtime;

use collectors::logger;
use collectors::Collector;

// mkfifo /tmp/test
// <some-executable> 2> >(sed 's/^/stderr: /' > /tmp/test) > >(sed 's/^/stdout: /' > /tmp/test) &

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut rt = runtime::Runtime::new();

    let logs_collector = Collector::Logs(
        logger::LogsCollectorBuilder::new()
            .with_tailer(logger::Tailer::new_np(
                "/tmp/test".to_string(),
                rt.get_transmitter(),
            ))
            .build()?,
    );
    rt.add_collector(logs_collector);

    rt.run().await;
    Ok(())
}

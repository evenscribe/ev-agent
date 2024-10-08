mod aggregator;
mod collectors;
mod event;
mod runtime;

#[tokio::main]
async fn main() {
    let mut rt = runtime::Runtime::new();

    //TODO: There must be some better way of passing transmitters to the collectors but I am not sure how to do it.
    rt.add_collector(Box::new(collectors::LogsCollector::new(
        rt.get_transmitter(),
    )));
    rt.add_collector(Box::new(collectors::MetricsCollector::new(
        rt.get_transmitter(),
    )));

    rt.run().await;
}

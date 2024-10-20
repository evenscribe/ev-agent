mod agent;
mod aggregator;
mod collectors;
mod event;
mod integration;
mod runtime;
mod service;

use agent::Agent;
// mkfifo /tmp/test
// <some-executable> 2> >(sed 's/^/stderr: /' > /tmp/test) > >(sed 's/^/stdout: /' > /tmp/test) &

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let agent = Agent::new();
    println!("{:?}", agent);
    Ok(())
}

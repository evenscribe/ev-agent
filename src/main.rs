mod agent;
mod event;
mod integration;
mod service;

use agent::Agent;
// mkfifo /tmp/test
// <some-executable> 2> >(sed 's/^/stderr: /' > /tmp/test) > >(sed 's/^/stdout: /' > /tmp/test) &

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut agent = Agent::new().await;
    agent.run().await?;
    println!("{:?}", agent);
    Ok(())
}

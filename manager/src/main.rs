use manager::gateway;
use manager::api::{Runner, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let gateway_controller = gateway::Controller{};
    gateway_controller.run().await
}

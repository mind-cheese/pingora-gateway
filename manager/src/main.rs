use kube::Client;
use manager::api::{Context, Result, Runner};
use manager::gateway;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::try_default().await.unwrap();

    let gateway_controller = gateway::Controller::new(Context { client });
    gateway_controller.run().await
}

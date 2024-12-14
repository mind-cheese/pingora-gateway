use api;
use api::{spec::init_deployment_template, Context, Result, Runner};
use k8s_openapi::api::apps::v1::Deployment;
use kube::Client;
use manager::gateway;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber).unwrap();
    let client = Client::try_default().await?;
    let deployment_template_path =
        "/Users/yangs/Project/cheeseforce/pingora-gateway/config/gateway/deployment–template.yaml";
    init_deployment_template(&deployment_template_path)?;
    let gateway_controller = gateway::Controller::builder()
        .ctx(Context { client })
        .build();
    info!("starting controller");
    gateway_controller.run().await
}

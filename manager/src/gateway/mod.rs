use crate::api::labels::MANAGERED_BY_CHEESEFORCE_GATEWAY_LABEL;
use crate::api::*;
use async_trait::async_trait;
use error::Error;
use futures::StreamExt;
use gateway_api::apis::standard::gateways::Gateway;
use k8s_openapi::api::apps::v1::Deployment;
use kube::api::Api;
use kube::core::Selector;
use kube::runtime;
use kube::runtime::{controller::Action, watcher::Config};
use std::{sync::Arc, time::Duration};
use tracing::*;

pub struct Controller {
    ctx: Arc<Context>,
    inner: runtime::Controller<Gateway>,
}

#[async_trait]
impl Runner for Controller {
    async fn run(&self) -> Result<()> {
        self.inner
            .shutdown_on_signal()
            .run(reconcile, error_policy, self.ctx.clone())
            .for_each(|res| async move {
                match res {
                    Ok(o) => info!("reconciled {:?}", o),
                    Err(e) => warn!("reconcile failed: {}", e),
                }
            })
            .await;
        Ok(())
    }
}

impl Controller {
    fn new(ctx: Context) -> Self {
        let gateway: Api<Gateway> = Api::all(ctx.client.clone());
        let deploy: Api<Deployment> = Api::all(ctx.client.clone());
        let managed_label = MANAGERED_BY_CHEESEFORCE_GATEWAY_LABEL.clone();
        let maganed_deploy_selector = Selector::from_iter(managed_label);

        Self {
            ctx: Arc::new(ctx),
            inner: runtime::Controller::new(
                gateway,
                Config::default().timeout(5 * 60).any_semantic(),
            )
            .owns(
                deploy,
                Config::default()
                    .timeout(5 * 60)
                    .labels_from(&maganed_deploy_selector),
            ),
        }
    }
}

async fn reconcile(gateway: Arc<Gateway>, ctx: Arc<Context>) -> Result<Action> {
    todo!()
}

fn error_policy(_: Arc<Gateway>, _: &Error, _: Arc<Context>) -> Action {
    Action::requeue(Duration::from_secs(5))
}

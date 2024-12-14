use api::labels::MANAGERED_BY_CHEESEFORCE_GATEWAY_LABEL;
use api::spec::GATEWAY_DEPLOYMENT_TEMPLATE;
use api::{Context, Result, Runner};
use async_trait::async_trait;
use bon::bon;
use error::Error;
use futures::StreamExt;
use gateway_api::apis::standard::gateways::Gateway;
use k8s_openapi::api::apps::v1::Deployment;
use kube::api::{Api, ObjectMeta, PostParams, Resource};
use kube::core::Selector;
use kube::runtime;
use kube::runtime::{controller::Action, watcher::Config};
use std::{sync::Arc, time::Duration};
use tracing::*;

pub struct Controller {
    ctx: Context,
    inner: runtime::Controller<Gateway>,
}

#[async_trait]
impl Runner for Controller {
    async fn run(self) -> Result<()> {
        self.inner
            .shutdown_on_signal()
            .run(reconcile, error_policy, self.ctx.into())
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

#[bon]
impl Controller {
    #[builder]
    pub fn new(ctx: Context) -> Self {
        let gateway: Api<Gateway> = Api::all(ctx.client.clone());
        let deploy: Api<Deployment> = Api::all(ctx.client.clone());
        let maganed_deploy_selector =
            Selector::from_iter(MANAGERED_BY_CHEESEFORCE_GATEWAY_LABEL.clone());
        let watch_config = Config::default().timeout(5 * 60);
        Self {
            ctx: ctx.clone(),
            inner: runtime::Controller::new(gateway, watch_config.clone().any_semantic()).owns(
                deploy,
                watch_config.clone().labels_from(&maganed_deploy_selector),
            ),
        }
    }
}

async fn reconcile(gateway: Arc<Gateway>, ctx: Arc<Context>) -> Result<Action> {
    let client = &ctx.client;
    deploy_gateway(client, gateway.clone()).await?;
    Ok(Action::await_change())
}

fn error_policy(_: Arc<Gateway>, _: &Error, _: Arc<Context>) -> Action {
    Action::requeue(Duration::from_secs(5))
}

async fn deploy_gateway(client: &kube::Client, gateway: Arc<Gateway>) -> Result<()> {
    let mut gateway_deployment = match GATEWAY_DEPLOYMENT_TEMPLATE.get().cloned() {
        Some(template) => template,
        None => return Err(Error::TemplateInitError),
    };
    let oref = gateway.controller_owner_ref(&()).unwrap();
    gateway_deployment.metadata = ObjectMeta {
        name: gateway.metadata.name.clone(),
        namespace: gateway.metadata.namespace.clone(),
        owner_references: Some(vec![oref]),
        ..ObjectMeta::default()
    };
    let deployments = Api::<Deployment>::namespaced(
        client.clone(),
        gateway
            .metadata
            .namespace
            .as_ref()
            .ok_or_else(|| Error::MissingObjectKey)?,
    );
    deployments
        .create(&PostParams::default(), &gateway_deployment)
        .await?;
    Ok(())
}

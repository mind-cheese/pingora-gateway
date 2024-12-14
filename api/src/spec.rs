use crate::Result;
use k8s_openapi::api::apps::v1::Deployment;
use once_cell::sync::OnceCell;
use serde::de::DeserializeOwned;
use serde_yaml;
use std::fs;
use std::path::Path;

pub const GATEWAY_CLASS_CONTROLLER_NAME: &str = "gateway.networking.k8s.io/cheeseforce";

pub static GATEWAY_DEPLOYMENT_TEMPLATE: OnceCell<Deployment> = OnceCell::new();

pub fn init_deployment_template(path: &impl AsRef<Path>) -> Result<()> {
    let deploy_template = get_template_from_yaml::<Deployment>(path)?;
    GATEWAY_DEPLOYMENT_TEMPLATE.set(deploy_template).unwrap();
    Ok(())
}

pub fn get_template_from_yaml<T>(path: &impl AsRef<Path>) -> Result<T>
where
    T: DeserializeOwned,
{
    let yaml = fs::File::open(path)?;
    let template = serde_yaml::from_reader::<fs::File, T>(yaml)?;
    Ok(template)
}

use k8s_openapi::api::apps::v1::Deployment;
use lazy_static::lazy_static;
use std::collections::BTreeMap;

lazy_static! {
    pub static ref MANAGERED_BY_CHEESEFORCE_GATEWAY_LABEL: BTreeMap<String, String> = {
        [(
            "gateway.networking.k8s.io/managed-by".to_string(),
            "cheeseforce".to_string(),
        )]
        .into()
    };
    pub static ref GATEWAY_DEPLOYMENT_TEMPLATE: Deployment = Deployment::default();
}

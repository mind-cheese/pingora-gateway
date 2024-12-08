use async_trait::async_trait;
use error::Error;
use kube::Client;

pub mod labels;
pub mod spec;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone)]
pub struct Context {
    pub client: Client,
}

#[async_trait]
pub trait Runner {
    async fn run(&self) -> Result<()>;
}

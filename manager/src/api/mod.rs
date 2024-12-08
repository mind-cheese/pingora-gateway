use async_trait::async_trait;
use error::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[async_trait]
pub trait Runner {
    async fn run(&self) -> Result<()>;
}
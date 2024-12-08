use async_trait::async_trait;
use crate::api::*;
pub struct Controller {}

#[async_trait]
impl Runner for Controller {
    async fn run(&self) -> Result<()> {
       Ok(())
    }
}

impl Controller {
    fn new() -> Self {

    }
}
pub mod check_config;
pub mod compile;

use async_trait::async_trait;

#[async_trait]
pub trait CliRun {
    async fn run(&self) -> u8;
}

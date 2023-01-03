pub mod compile;
pub mod config_check;

use async_trait::async_trait;

#[async_trait]
pub trait CliRun {
    async fn run(&self) -> u8;
}

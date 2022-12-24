use thiserror::Error;

use crate::{
    cli::ConfigUrl,
    fetch::{Fetch, FetchError},
};

pub struct FetchConfig(Fetch);

#[derive(Error, Debug)]
#[error("FetchConfigError: {0}")]
pub struct FetchConfigError(#[from] FetchError);

impl FetchConfig {
    pub async fn fetch(&self) -> Result<String, FetchConfigError> {
        Ok(self.0.fetch().await?)
    }
}

impl From<&ConfigUrl> for FetchConfig {
    fn from(config_url: &ConfigUrl) -> Self {
        match config_url {
            ConfigUrl::Url(url) => Self(Fetch::from(url.clone())),
            ConfigUrl::File(path) => Self(Fetch::from(path.clone())),
        }
    }
}

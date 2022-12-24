use thiserror::Error;

use crate::{
    cli::ConfigUrl,
    fetch::{Fetch, FetchError},
};

use super::{Config, SourceConfig};

pub struct ConfigProvider {
    config_url: ConfigUrl,
    fetch_file: Fetch,
}

#[derive(Error, Debug)]
pub enum LoadConfigError {
    #[error("FetchError: {0}")]
    Fetch(#[from] FetchError),

    #[error("ParseError: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

impl ConfigProvider {
    pub async fn load(&self) -> Result<Config, LoadConfigError> {
        let content = self.fetch_file.fetch().await?;
        let source_config: SourceConfig = serde_yaml::from_str(&content)?;
        let config = Config {
            config_url: self.config_url.clone(),
            blacklist: source_config.blacklist,
            whitelist: source_config.whitelist,
            overrides: source_config.overrides,
        };

        Ok(config)
    }
}

impl From<&ConfigUrl> for ConfigProvider {
    fn from(config_url: &ConfigUrl) -> Self {
        Self {
            config_url: config_url.to_owned(),
            fetch_file: config_url.into(),
        }
    }
}

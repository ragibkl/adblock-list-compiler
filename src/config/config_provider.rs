use thiserror::Error;

use crate::cli::ConfigUrl;

use super::{
    fetch_config::{FetchConfig, FetchConfigError},
    Config, SourceConfig,
};

pub struct ConfigProvider {
    config_url: ConfigUrl,
    fetch_config: FetchConfig,
}

#[derive(Error, Debug)]
pub enum LoadConfigError {
    #[error("FetchError: {0}")]
    Fetch(#[from] FetchConfigError),

    #[error("ParseError: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

impl ConfigProvider {
    pub async fn load(&self) -> Result<Config, LoadConfigError> {
        let content = self.fetch_config.fetch().await?;
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
            config_url: config_url.clone(),
            fetch_config: config_url.into(),
        }
    }
}

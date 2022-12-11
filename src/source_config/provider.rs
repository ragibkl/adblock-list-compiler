use std::{fs, path::PathBuf};

use thiserror::Error;
use url::Url;

use super::source_config::SourceConfig;

pub enum SourceConfigProvider {
    HTTPProvider { url: Url },
    FileProvider { path: PathBuf },
}

#[derive(Error, Debug)]
pub enum FetchConfigError {
    #[error("HTTPError: {0}")]
    HTTPError(#[from] reqwest::Error),

    #[error("FileReadError: {0}")]
    FileReadError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum LoadConfigError {
    #[error("FetchError: {0}")]
    FetchConfigError(#[from] FetchConfigError),

    #[error("ParseError: {0}")]
    ParseJsonError(#[from] serde_json::Error),

    #[error("ParseError: {0}")]
    ParseYamlError(#[from] serde_yaml::Error),
}

impl SourceConfigProvider {
    async fn fetch(&self) -> Result<String, FetchConfigError> {
        let content = match self {
            SourceConfigProvider::HTTPProvider { url } => {
                reqwest::get(url.clone()).await?.text().await?
            }
            SourceConfigProvider::FileProvider { path } => fs::read_to_string(path.clone())?,
        };

        Ok(content)
    }

    pub async fn load_config(&self) -> Result<SourceConfig, LoadConfigError> {
        let content = self.fetch().await?;
        let source_config: SourceConfig = serde_yaml::from_str(&content)?;
        Ok(source_config)
    }
}

impl From<&PathBuf> for SourceConfigProvider {
    fn from(path: &PathBuf) -> Self {
        Self::FileProvider { path: path.clone() }
    }
}

impl From<&Url> for SourceConfigProvider {
    fn from(url: &Url) -> Self {
        Self::HTTPProvider { url: url.clone() }
    }
}

use std::{fs, path::PathBuf};

use thiserror::Error;
use url::Url;

use super::Config;

pub enum SourceConfigProvider {
    HTTPProvider { url: Url },
    FileProvider { path: PathBuf },
}

#[derive(Error, Debug)]
pub enum FetchConfigError {
    #[error("HTTPError: {0}")]
    Http(#[from] reqwest::Error),

    #[error("FileReadError: {0}")]
    FileRead(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum LoadConfigError {
    #[error("FetchError: {0}")]
    Fetch(#[from] FetchConfigError),

    #[error("ParseError: {0}")]
    ParseJson(#[from] serde_json::Error),

    #[error("ParseError: {0}")]
    ParseYaml(#[from] serde_yaml::Error),
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

    pub async fn load_config(&self) -> Result<Config, LoadConfigError> {
        let content = self.fetch().await?;
        let source_config: Config = serde_yaml::from_str(&content)?;
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

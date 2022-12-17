use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

use thiserror::Error;
use url::Url;

use crate::cli::ConfigUrl;

#[derive(Debug)]
pub struct FetchHTTP {
    pub url: Url,
}

#[derive(Error, Debug)]
pub enum FetchHTTPError {
    #[error("HTTPError: {0}")]
    HTTPError(#[from] reqwest::Error),
}

impl FetchHTTP {
    async fn fetch(&self) -> Result<String, FetchHTTPError> {
        let response = reqwest::get(self.url.to_string()).await?;
        let text = response.text().await?;
        Ok(text)
    }
}

#[derive(Debug)]
pub struct FetchFile {
    pub path: PathBuf,
}

#[derive(Error, Debug)]
pub enum FetchFileError {
    #[error("FileError: {0}")]
    FileError(#[from] std::io::Error),
}

impl FetchFile {
    async fn fetch(&self) -> Result<String, FetchFileError> {
        let contents = read_to_string(&self.path)?;
        Ok(contents)
    }
}

#[derive(Debug)]
pub enum FetchSource {
    HTTP(FetchHTTP),
    File(FetchFile),
}

#[derive(Error, Debug)]
pub enum FetchSourceError {
    #[error("HTTPError: {0}")]
    HTTPError(#[from] FetchHTTPError),

    #[error("FileError: {0}")]
    FileError(#[from] FetchFileError),
}

impl FetchSource {
    pub fn new_from(source_path: &str, config_url: &ConfigUrl) -> Self {
        if source_path.starts_with("http") {
            let u = url::Url::parse(source_path).unwrap();
            FetchSource::HTTP(FetchHTTP { url: u })
        } else if source_path.starts_with("./") {
            match config_url {
                ConfigUrl::Url(u) => {
                    let a = u.join(source_path).unwrap();
                    FetchSource::HTTP(FetchHTTP { url: a })
                }
                ConfigUrl::File(p) => {
                    let q = p.join(source_path);
                    FetchSource::File(FetchFile { path: q })
                }
            }
        } else {
            let path = PathBuf::from_str(source_path).unwrap();
            FetchSource::File(FetchFile { path })
        }
    }

    pub async fn fetch(&self) -> Result<String, FetchSourceError> {
        match self {
            FetchSource::HTTP(p) => Ok(p.fetch().await?),
            FetchSource::File(p) => Ok(p.fetch().await?),
        }
    }
}

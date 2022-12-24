mod file;
mod http;

use std::path::PathBuf;
use std::str::FromStr;

use thiserror::Error;
use url::Url;

use crate::cli::ConfigUrl;

use self::{
    file::{FetchFile, FetchFileError},
    http::{FetchHTTPError, FetchHttp},
};

#[derive(Debug)]
pub enum FetchSource {
    Http(FetchHttp),
    File(FetchFile),
}

#[derive(Error, Debug)]
pub enum FetchSourceError {
    #[error("HTTPError: {0}")]
    HTTPError(#[from] FetchHTTPError),

    #[error("FileError: {0}")]
    FileError(#[from] FetchFileError),
}

#[derive(Error, Debug)]
pub enum FetchConfigError {
    #[error("InvalidUrl: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("InvalidPath")]
    InvalidPath,

    #[error("FileNotExists")]
    FileNotExists,

    #[error("Infallible")]
    Infallible(#[from] core::convert::Infallible),
}

impl FetchSource {
    pub fn try_from(source_path: &str, config_url: &ConfigUrl) -> Result<Self, FetchConfigError> {
        if source_path.starts_with("http") {
            let url = Url::parse(source_path)?;
            Ok(FetchSource::Http(FetchHttp { url }))
        } else if source_path.starts_with("./") {
            match config_url {
                ConfigUrl::Url(u) => {
                    let url = u.join(source_path)?;
                    Ok(FetchSource::Http(FetchHttp { url }))
                }
                ConfigUrl::File(p) => {
                    let path = p
                        .parent()
                        .ok_or(FetchConfigError::InvalidPath)?
                        .join(source_path);
                    if path.exists() {
                        Ok(FetchSource::File(FetchFile { path }))
                    } else {
                        Err(FetchConfigError::FileNotExists)
                    }
                }
            }
        } else {
            let path = PathBuf::from_str(source_path)?;
            if path.exists() {
                Ok(FetchSource::File(FetchFile { path }))
            } else {
                Err(FetchConfigError::FileNotExists)
            }
        }
    }

    pub async fn fetch(&self) -> Result<String, FetchSourceError> {
        match self {
            FetchSource::Http(p) => Ok(p.fetch().await?),
            FetchSource::File(p) => Ok(p.fetch().await?),
        }
    }
}

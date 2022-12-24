mod file;
mod http;

use thiserror::Error;

use crate::cli::ConfigUrl;

use self::{
    file::{FetchFile, FetchFileError},
    http::{FetchHTTPError, FetchHttp},
};

#[derive(Debug)]
pub enum Fetch {
    Http(FetchHttp),
    File(FetchFile),
}

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("HTTPError: {0}")]
    HTTPError(#[from] FetchHTTPError),

    #[error("FileError: {0}")]
    FileError(#[from] FetchFileError),
}

impl Fetch {
    pub async fn fetch(&self) -> Result<String, FetchError> {
        match self {
            Fetch::Http(p) => Ok(p.fetch().await?),
            Fetch::File(p) => Ok(p.fetch().await?),
        }
    }
}

impl From<&ConfigUrl> for Fetch {
    fn from(value: &ConfigUrl) -> Self {
        match value {
            ConfigUrl::Url(url) => Self::Http(FetchHttp {
                url: url.to_owned(),
            }),
            ConfigUrl::File(path) => Self::File(FetchFile {
                path: path.to_owned(),
            }),
        }
    }
}

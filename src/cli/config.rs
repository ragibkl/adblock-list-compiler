use std::{path::PathBuf, str::FromStr};

use thiserror::Error;
use url::Url;

#[derive(Clone, Debug)]
pub enum Config {
    Url(Url),
    File(PathBuf),
}

#[derive(Error, Debug)]
#[error("ParseConfigError")]
pub struct ParseConfigError;

impl FromStr for Config {
    type Err = ParseConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(url) = Url::parse(s) {
            return Ok(Self::Url(url));
        }

        if let Ok(path_buf) = PathBuf::from_str(s) {
            return Ok(Self::File(path_buf));
        }

        Err(ParseConfigError)
    }
}

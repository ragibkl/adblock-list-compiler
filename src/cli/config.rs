use std::{fmt::Display, path::PathBuf, str::FromStr};

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

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Config::Url(url) => f.write_fmt(format_args!("Url: {}", url.to_string())),
            Config::File(path) => f.write_fmt(format_args!(
                "File: {}",
                path.as_path().as_os_str().to_str().unwrap_or_default()
            )),
        }
    }
}

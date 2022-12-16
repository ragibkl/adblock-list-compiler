use std::fs::read_to_string;
use std::path::PathBuf;
use std::str::FromStr;

use url::Url;

use crate::cli::Config;

pub struct FetchHTTP {
    pub url: Url,
}
impl FetchHTTP {
    async fn fetch(&self) -> Result<String, ()> {
        let response = reqwest::get(self.url.to_string()).await.map_err(|e| {
            println!("failed http: {}", &self.url.to_string());
            ()
        })?;
        let text = response.text().await.map_err(|e| ())?;
        Ok(text)
    }
}

pub struct FetchFile {
    pub path: PathBuf,
}

impl FetchFile {
    async fn fetch(&self) -> Result<String, ()> {
        let contents = read_to_string(&self.path).map_err(|e| {
            println!("failed file: {}", &self.path.as_path().to_str().unwrap());
            ()
        })?;
        Ok(contents)
    }
}

pub enum FetchSource {
    HTTP(FetchHTTP),
    File(FetchFile),
}

impl FetchSource {
    pub fn new_from(source_path: &str, config_url: &Config) -> Self {
        if source_path.starts_with("http") {
            let u = url::Url::parse(source_path).unwrap();
            FetchSource::HTTP(FetchHTTP { url: u })
        } else if source_path.starts_with("./") {
            match config_url {
                Config::Url(u) => {
                    let a = u.join(source_path).unwrap();
                    FetchSource::HTTP(FetchHTTP { url: a })
                }
                Config::File(p) => {
                    let q = p.join(source_path);
                    FetchSource::File(FetchFile { path: q })
                }
            }
        } else {
            let path = PathBuf::from_str(source_path).unwrap();
            FetchSource::File(FetchFile { path })
        }
    }

    pub async fn fetch(&self) -> Result<String, ()> {
        match self {
            FetchSource::HTTP(p) => p.fetch().await,
            FetchSource::File(p) => p.fetch().await,
        }
    }
}

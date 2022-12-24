use std::time::Duration;

use lazy_static::lazy_static;
use thiserror::Error;
use url::Url;

#[derive(Debug)]
pub struct FetchHttp {
    pub url: Url,
}

#[derive(Error, Debug)]
pub enum FetchHTTPError {
    #[error("HTTPError: {0}")]
    HTTPError(#[from] reqwest::Error),
}

impl FetchHttp {
    pub async fn fetch(&self) -> Result<String, FetchHTTPError> {
        lazy_static! {
            static ref CLIENT: reqwest::Client = reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(5))
                .timeout(Duration::from_secs(60))
                .build()
                .unwrap();
        }

        let response = CLIENT.get(self.url.to_string()).send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}

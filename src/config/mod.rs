use crate::cli::ConfigUrl;

mod config_provider;
// pub mod provider;

pub use config_provider::ConfigProvider;

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BlacklistFormat {
    Hosts,
    Domains,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum WhitelistFormat {
    Hosts,
    Domains,
    Zone,
}

#[derive(serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum OverrideFormat {
    Cname,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Source<T> {
    pub format: T,
    pub path: String,
}

pub type Blacklist = Source<BlacklistFormat>;
pub type Whitelist = Source<WhitelistFormat>;
pub type Overrides = Source<OverrideFormat>;

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SourceConfig {
    pub blacklist: Vec<Blacklist>,
    pub whitelist: Vec<Whitelist>,
    pub overrides: Vec<Overrides>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub config_url: ConfigUrl,
    pub blacklist: Vec<Blacklist>,
    pub whitelist: Vec<Whitelist>,
    pub overrides: Vec<Overrides>,
}

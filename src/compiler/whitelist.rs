use crate::config::WhitelistFormat;

use super::{
    fetch_source::FetchSource,
    parser::{CName, Domain, Host},
};

#[derive(Debug)]
pub enum ParseWhitelist {
    Hosts,
    Domains,
    Zone,
}

impl ParseWhitelist {
    pub fn parse(&self, value: &str) -> Option<Domain> {
        match self {
            ParseWhitelist::Hosts => Host::parse(value).map(|h| h.into_domain()),
            ParseWhitelist::Domains => Domain::parse(value),
            ParseWhitelist::Zone => CName::parse(value).map(|c| c.into_domain()),
        }
    }
}

impl From<&WhitelistFormat> for ParseWhitelist {
    fn from(value: &WhitelistFormat) -> Self {
        match value {
            WhitelistFormat::Hosts => ParseWhitelist::Hosts,
            WhitelistFormat::Domains => ParseWhitelist::Domains,
            WhitelistFormat::Zone => ParseWhitelist::Zone,
        }
    }
}

#[derive(Debug)]
pub struct WhitelistCompiler {
    pub(crate) source: FetchSource,
    pub(crate) parser: ParseWhitelist,
}

impl WhitelistCompiler {
    pub async fn load_whitelist(&self) -> Vec<Domain> {
        let source = self.source.fetch().await.unwrap();

        let mut domains: Vec<Domain> = Vec::new();
        for line in source.lines() {
            if let Some(bl) = self.parser.parse(line) {
                domains.push(bl);
            }
        }

        domains
    }
}

mod cname;
mod domain;
mod host;

use crate::source_config::source_config::{BlacklistFormat, WhitelistFormat};

pub use self::domain::Domain;
use self::{cname::CName, host::Host};

pub enum ParseBlacklist {
    Hosts,
    Domains,
}

impl ParseBlacklist {
    pub fn parse(&self, value: &str) -> Option<Domain> {
        match self {
            ParseBlacklist::Hosts => Host::parse(value).map(|h| h.into_domain()),
            ParseBlacklist::Domains => Domain::parse(value),
        }
    }
}

impl From<&BlacklistFormat> for ParseBlacklist {
    fn from(value: &BlacklistFormat) -> Self {
        match value {
            BlacklistFormat::Hosts => ParseBlacklist::Hosts,
            BlacklistFormat::Domains => ParseBlacklist::Domains,
        }
    }
}

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

pub enum ParseOverride {
    CName,
}

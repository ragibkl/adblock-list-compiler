mod cname;
mod domains;
mod hosts;

use crate::source_config::source_config::BlacklistFormat;

pub use self::{domains::DomainsParser, hosts::HostsParser};

#[derive(Debug, PartialEq)]
pub struct Domain(pub String);

#[derive(Debug, PartialEq)]
pub struct Override {
    pub from: String,
    pub to: String,
}

pub struct CNameParser;
pub struct ZoneParser;

pub enum BlacklistParser {
    Hosts(HostsParser),
    Domains(DomainsParser),
}

impl BlacklistParser {
    pub fn parse(&self, value: &str) -> Option<Domain> {
        match self {
            BlacklistParser::Hosts(p) => p.parse(value),
            BlacklistParser::Domains(p) => p.parse(value),
        }
    }
}

impl From<&BlacklistFormat> for BlacklistParser {
    fn from(value: &BlacklistFormat) -> Self {
        match value {
            BlacklistFormat::Hosts => BlacklistParser::Hosts(HostsParser {}),
            BlacklistFormat::Domains => BlacklistParser::Domains(DomainsParser {}),
        }
    }
}

pub enum WhitelistFormat {
    HostsParser(HostsParser),
    DomainsParser(DomainsParser),
    CNameParser(CNameParser),
    ZoneParser(ZoneParser),
}

pub enum OverrideFormat {
    CNameParser(CNameParser),
}

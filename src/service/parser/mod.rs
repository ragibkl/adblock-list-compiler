mod cname;
mod domains;
mod hosts;
mod zone;

use crate::source_config::source_config::{BlacklistFormat, WhitelistFormat};

use self::{cname::CNameParser, domains::DomainsParser, hosts::HostsParser, zone::ZoneParser};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Domain(pub String);

#[derive(Debug, PartialEq)]
pub struct Override {
    pub from: String,
    pub to: String,
}

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

pub enum WhitelistParser {
    HostsParser(HostsParser),
    DomainsParser(DomainsParser),
    ZoneParser(ZoneParser),
}

impl WhitelistParser {
    pub fn parse(&self, value: &str) -> Option<Domain> {
        match self {
            WhitelistParser::HostsParser(p) => p.parse(value),
            WhitelistParser::DomainsParser(p) => p.parse(value),
            WhitelistParser::ZoneParser(p) => p.parse(value),
        }
    }
}

impl From<&WhitelistFormat> for WhitelistParser {
    fn from(value: &WhitelistFormat) -> Self {
        match value {
            WhitelistFormat::Hosts => WhitelistParser::HostsParser(HostsParser {}),
            WhitelistFormat::Domains => WhitelistParser::DomainsParser(DomainsParser {}),
            WhitelistFormat::Zone => WhitelistParser::ZoneParser(ZoneParser {}),
        }
    }
}

pub enum OverrideFormat {
    CNameParser(CNameParser),
}

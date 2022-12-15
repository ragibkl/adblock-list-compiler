use addr::parse_domain_name;
use lazy_static::lazy_static;
use regex::Regex;

use super::Domain;

pub struct HostsParser;

impl HostsParser {
    pub(super) fn parse(&self, value: &str) -> Option<Domain> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(127\.0\.0\.1|0\.0\.0\.0)\s+(?P<domain>.{2,200}\.[a-z]{2,6})")
                    .unwrap();
        }

        RE.captures(value)
            .and_then(|cap| cap.name("domain"))
            .and_then(|d| parse_domain_name(d.as_str()).ok())
            .map(|d| Domain(d.as_str().trim().to_string()))
    }
}

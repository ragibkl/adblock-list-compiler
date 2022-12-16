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
            .map(|d| d.as_str().trim().to_string())
            .map(|d| idna::domain_to_ascii(&d).ok())
            .flatten()
            .map(|d| Domain(d))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_domain() {
        let parser = HostsParser {};

        let input = "127.0.0.1 abc.example.com";
        let expected = Domain("abc.example.com".to_string());
        let output = parser.parse(input);
        assert_eq!(output, Some(expected));

        let input = "127.0.0.1 Bücher.example.com";
        let expected = Domain("xn--bcher-kva.example.com".to_string());
        let output = parser.parse(input);
        assert_eq!(output, Some(expected));
    }
}

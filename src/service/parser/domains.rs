use addr::parse_domain_name;
use lazy_static::lazy_static;
use regex::Regex;

use super::Domain;

pub struct DomainsParser;

impl DomainsParser {
    pub(super) fn parse(&self, value: &str) -> Option<Domain> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?P<domain>.{2,200}\.[a-z]{2,6})").unwrap();
        }

        RE.captures(&value)
            .and_then(|cap| cap.name("domain"))
            .and_then(|d| {
                if d.as_str().starts_with("*.") {
                    let as_str = d.as_str().replace("*.", "");
                    parse_domain_name(&as_str)
                        .ok()
                        .map(|v| "*.".to_string() + v.as_str())
                } else {
                    parse_domain_name(d.as_str())
                        .ok()
                        .map(|v| v.as_str().trim().to_string())
                }
            })
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
        let parser = DomainsParser {};

        let input = "abc.example.com";
        let expected = Domain("abc.example.com".to_string());
        let output = parser.parse(input);
        assert_eq!(output, Some(expected));

        let input = "Bücher.example.com";
        let expected = Domain("xn--bcher-kva.example.com".to_string());
        let output = parser.parse(input);
        assert_eq!(output, Some(expected));

        let input = "";
        let output = parser.parse(input);
        assert_eq!(output, None);
    }
}

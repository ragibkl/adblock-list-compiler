use addr::parse_domain_name;
use lazy_static::lazy_static;
use regex::Regex;

use super::Override;

pub struct CNameParser;

impl CNameParser {
    pub(super) fn parse(&self, value: &str) -> Option<Override> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(?P<domain>.{2,200}\.[a-z]{2,6})\s+(CNAME|cname)\s+(?P<alias>.{2,200}\.[a-z]{2,6})\."
            )
            .unwrap();
        }

        RE.captures(&value)
            .and_then(|cap| {
                let domain = cap.name("domain");
                let alias = cap.name("alias");

                match (domain, alias) {
                    (Some(d), Some(a)) => Some((d, a)),
                    _ => None,
                }
            })
            .and_then(|(d, a)| {
                let domain = parse_domain_name(d.as_str()).ok();
                let alias = parse_domain_name(a.as_str()).ok();
                match (domain, alias) {
                    (Some(d), Some(a)) => Some(Override {
                        from: d.to_string(),
                        to: a.to_string(),
                    }),
                    _ => None,
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_extract_domain() {
        let parser = CNameParser {};

        let input = "www.bing.com    CNAME   strict.bing.com.";
        let expected = Override {
            from: "www.bing.com".to_string(),
            to: "strict.bing.com".to_string(),
        };
        let output = parser.parse(input);
        assert_eq!(output, Some(expected));

        // todo: idna domain
    }
}

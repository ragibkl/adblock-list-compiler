use std::fmt::Display;

use super::parser::Domain;

pub enum OutputFormat {
    Zone,
}

impl OutputFormat {
    pub fn write(&self, domains: Vec<Domain>) -> String {
        let mut lines: Vec<String> = Vec::new();
        lines.push("$TTL 1H".to_string());
        lines.push(
            "@               SOA     LOCALHOST. named-mgr.example.com (1 1h 15m 30d 2h)"
                .to_string(),
        );
        lines.push("                NS      LOCALHOST.".to_string());

        let domain_lines = domains
            .iter()
            .map(|s| format!("{} CNAME null.null-zone.null.", s.0));

        lines.join("\n")
    }
}

impl Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

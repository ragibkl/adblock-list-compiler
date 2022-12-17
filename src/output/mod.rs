use std::fmt::Display;

use crate::compiler::Adblock;

pub struct ZoneOutput {
    adblock: Adblock,
}

impl ZoneOutput {
    pub fn new(adblock: Adblock) -> Self {
        Self { adblock }
    }

    fn format_blacklist(&self, bl: &str) -> String {
        format!("{} CNAME null.null-zone.null.", bl)
    }

    fn format_cname_override(&self, domain: &str, alias: &str) -> String {
        format!("{} CNAME {}.", domain, alias)
    }

    pub fn build_string(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        lines.push("$TTL 1H".to_string());
        lines.push(
            "@               SOA     LOCALHOST. named-mgr.example.com (1 1h 15m 30d 2h)"
                .to_string(),
        );
        lines.push("                NS      LOCALHOST.".to_string());

        for domain in &self.adblock.blacklists {
            let line = self.format_blacklist(&domain.0);
            lines.push(line);
        }

        for cname in &self.adblock.overrides {
            let line = self.format_cname_override(&cname.domain.0, &cname.alias.0);
            lines.push(line);
        }

        lines.join("\n")
    }
}

impl Display for ZoneOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.build_string())
    }
}

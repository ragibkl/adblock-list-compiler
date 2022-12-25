use std::{fmt::Display, fs::File, io::Write, path::Path};

use crate::compiler::Adblock;

pub struct ZoneOutput {
    adblock: Adblock,
}

fn format_blacklist(bl: &str) -> String {
    format!("{} CNAME null.null-zone.null.", bl)
}

fn format_cname_rewrite(domain: &str, alias: &str) -> String {
    format!("{} CNAME {}.", domain, alias)
}

impl ZoneOutput {
    pub fn new(adblock: Adblock) -> Self {
        Self { adblock }
    }

    pub fn write_all(self, path: &Path) -> Result<(), std::io::Error> {
        let mut f = File::create(path)?;

        writeln!(f, "$TTL 1H")?;
        writeln!(
            f,
            "@               SOA     LOCALHOST. named-mgr.example.com (1 1h 15m 30d 2h)"
        )?;
        writeln!(f, "                NS      LOCALHOST.")?;

        for cname in &self.adblock.rewrites {
            let line = format_cname_rewrite(&cname.domain.0, &cname.alias.0);
            writeln!(f, "{}", line)?;
        }

        for domain in &self.adblock.blacklists {
            let line = format_blacklist(&domain.0);
            writeln!(f, "{}", line)?;
        }

        // zone file must end with a newline
        writeln!(f)?;

        f.sync_all()?;
        Ok(())
    }
}

impl Display for ZoneOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "$TTL 1H")?;
        writeln!(
            f,
            "@               SOA     LOCALHOST. named-mgr.example.com (1 1h 15m 30d 2h)"
        )?;
        writeln!(f, "                NS      LOCALHOST.")?;

        for cname in &self.adblock.rewrites {
            let line = format_cname_rewrite(&cname.domain.0, &cname.alias.0);
            writeln!(f, "{}", line)?;
        }

        for domain in &self.adblock.blacklists {
            let line = format_blacklist(&domain.0);
            writeln!(f, "{}", line)?;
        }

        // zone file must end with a newline
        writeln!(f)?;

        Ok(())
    }
}

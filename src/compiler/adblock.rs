use std::collections::HashSet;

use thiserror::Error;

use crate::config::{Config, ConfigUrl};

use super::{
    blacklist::{BlacklistCompiler, ParseBlacklist},
    fetch_source::{FetchSource, FetchSourceInitError},
    parser::{CName, Domain},
    rewrites::{ParseRewrite, RewritesCompiler},
    whitelist::{ParseWhitelist, WhitelistCompiler},
};

pub struct Adblock {
    pub blacklists: Vec<Domain>,
    pub rewrites: Vec<CName>,
}

#[derive(Debug)]
pub struct AdblockCompiler {
    blacklists: Vec<BlacklistCompiler>,
    whitelists: Vec<WhitelistCompiler>,
    rewrites: Vec<RewritesCompiler>,
}

#[derive(Debug, Error)]
pub enum AdblockCompilerConfigError {
    #[error("{0}")]
    FetchSourceError(#[from] FetchSourceInitError),
}

impl AdblockCompiler {
    pub fn init(
        config: &Config,
        config_url: &ConfigUrl,
    ) -> Result<Self, AdblockCompilerConfigError> {
        let mut blacklists = Vec::new();

        for bl in &config.blacklist {
            let source = FetchSource::try_from(&bl.path, config_url)?;
            let parser = ParseBlacklist::from(&bl.format);

            blacklists.push(BlacklistCompiler { source, parser });
        }

        let mut whitelists = Vec::new();
        for wl in &config.whitelist {
            let source = FetchSource::try_from(&wl.path, config_url)?;
            let parser = ParseWhitelist::from(&wl.format);

            whitelists.push(WhitelistCompiler { source, parser });
        }

        let mut rewrites = Vec::new();
        for rw in &config.overrides {
            let source = FetchSource::try_from(&rw.path, config_url)?;
            let parser = ParseRewrite::from(&rw.format);

            rewrites.push(RewritesCompiler { source, parser });
        }

        Ok(Self {
            blacklists,
            whitelists,
            rewrites,
        })
    }

    pub async fn compile(&self) -> Adblock {
        let mut whitelists: HashSet<Domain> = HashSet::new();
        for wl in &self.whitelists {
            let domains = wl.load_whitelist().await;
            for d in domains {
                whitelists.insert(d);
            }
        }

        let mut blacklists: HashSet<Domain> = HashSet::new();
        for bl in &self.blacklists {
            let domains = bl.load_blacklist().await;

            for d in domains {
                if !whitelists.contains(&d) {
                    blacklists.insert(d);
                }
            }
        }

        let mut rewrites: Vec<CName> = Vec::new();
        for rw in &self.rewrites {
            let cnames = rw.load_rewrites().await;
            rewrites.extend(cnames);
        }

        let blacklists: Vec<Domain> = Vec::from_iter(blacklists);

        Adblock {
            blacklists,
            rewrites,
        }
    }
}

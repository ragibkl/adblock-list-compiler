use std::collections::HashSet;

use crate::{cli::ConfigUrl, source_config::source_config::SourceConfig};

use super::{
    blacklist::{BlacklistCompiler, ParseBlacklist},
    fetch_source::FetchSource,
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

impl AdblockCompiler {
    pub fn new(config: &SourceConfig, config_url: &ConfigUrl) -> Self {
        let blacklists: Vec<BlacklistCompiler> = config
            .blacklist
            .iter()
            .map(|bl| BlacklistCompiler {
                file_source: FetchSource::new_from(&bl.path, config_url),
                parser: ParseBlacklist::from(&bl.format),
            })
            .collect();

        let whitelists: Vec<WhitelistCompiler> = config
            .whitelist
            .iter()
            .map(|wl| WhitelistCompiler {
                file_source: FetchSource::new_from(&wl.path, config_url),
                parser: ParseWhitelist::from(&wl.format),
            })
            .collect();

        let rewrites: Vec<RewritesCompiler> = config
            .overrides
            .iter()
            .map(|rw| RewritesCompiler {
                file_source: FetchSource::new_from(&rw.path, config_url),
                parser: ParseRewrite::from(&rw.format),
            })
            .collect();

        Self {
            blacklists,
            whitelists,
            rewrites,
        }
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
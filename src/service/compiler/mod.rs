pub mod blacklist;
pub mod whitelist;

use std::collections::HashSet;

use crate::{cli::Config, source_config::source_config::SourceConfig};

use super::{
    fetch_source::FetchSource,
    parser::{BlacklistParser, Domain, WhitelistParser},
};

use self::{blacklist::BlacklistCompiler, whitelist::WhitelistCompiler};

pub struct AdblockCompiler {
    blacklists: Vec<BlacklistCompiler>,
    whitelists: Vec<WhitelistCompiler>,
}

impl AdblockCompiler {
    pub fn new(config: &SourceConfig, config_url: &Config) -> Self {
        let blacklists: Vec<BlacklistCompiler> = config
            .blacklist
            .iter()
            .map(|bl| BlacklistCompiler {
                file_source: FetchSource::new_from(&bl.path, config_url),
                parser: BlacklistParser::from(&bl.format),
            })
            .collect();

        let whitelists: Vec<WhitelistCompiler> = config
            .whitelist
            .iter()
            .map(|wl| WhitelistCompiler {
                file_source: FetchSource::new_from(&wl.path, config_url),
                parser: WhitelistParser::from(&wl.format),
            })
            .collect();

        Self {
            blacklists,
            whitelists,
        }
    }

    pub async fn compile(&self) {
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

        let blacklists: Vec<Domain> = Vec::from_iter(blacklists);

        println!("{:#?}", blacklists);
    }
}

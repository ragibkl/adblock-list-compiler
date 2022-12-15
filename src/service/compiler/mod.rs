pub mod blacklist;

use crate::{cli::Config, source_config::source_config::SourceConfig};

use super::{fetch_source::FetchSource, parser::BlacklistParser};

pub use self::blacklist::BlacklistCompiler;

pub struct AdblockCompiler {
    blacklists: Vec<BlacklistCompiler>,
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

        Self { blacklists }
    }
}

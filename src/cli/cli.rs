use clap::{Parser, Subcommand};

use super::config::Config;

#[derive(Debug, Subcommand)]
enum Command {
    /// Outputs the current config
    TestConfig {
        /// Sets a custom config file
        #[arg(
            short,
            long,
            value_name = "CONFIG",
            default_value = "https://raw.githubusercontent.com/ragibkl/adblock-dns-server/master/data/configuration.yaml"
        )]
        config: Config,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn from_args() -> Self {
        Self::parse()
    }
}

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use super::config::Config;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Outputs the current config
    CheckConfig {
        /// Sets a custom config file
        #[arg(
            short,
            long,
            value_name = "CONFIG",
            default_value = "https://raw.githubusercontent.com/ragibkl/adblock-dns-server/master/data/configuration.yaml"
        )]
        config: Config,

        /// output file location
        #[arg(short, long, value_name = "CONFIG", default_value = "./blacklist.zone")]
        output: PathBuf,

        /// output format
        #[arg(short, long, value_name = "FORMAT", default_value = "zone")]
        format: String,
    },
}

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn from_args() -> Self {
        Self::parse()
    }
}

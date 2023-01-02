use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{
    cli_run::{check_config::ConfigCheck, compile::Compile, CliRun},
    config::ConfigUrl,
};

#[derive(Debug, Subcommand)]
pub enum ConfigCommand {
    /// Outputs the current config
    Check {
        /// Sets a custom config file
        #[arg(
            short,
            long,
            value_name = "CONFIG",
            default_value = "https://raw.githubusercontent.com/ragibkl/adblock-dns-server/master/data/configuration.yaml"
        )]
        config_url: ConfigUrl,

        /// output file location
        #[arg(short, long, value_name = "CONFIG", default_value = "./blacklist.zone")]
        output: PathBuf,

        /// output format
        #[arg(short, long, value_name = "FORMAT", default_value = "zone")]
        format: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(subcommand)]
    Config(ConfigCommand),

    // Compiles an adblock list and output to file
    Compile {
        /// Sets a custom config file
        #[arg(
            short,
            long,
            value_name = "CONFIG",
            default_value = "https://raw.githubusercontent.com/ragibkl/adblock-dns-server/master/data/configuration.yaml"
        )]
        config_url: ConfigUrl,

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

    pub fn into_cli_run(self) -> Box<dyn CliRun> {
        match &self.command {
            Command::Config(c) => match c {
                ConfigCommand::Check {
                    config_url,
                    output,
                    format,
                } => Box::new(ConfigCheck::new(config_url, output, format)),
            },
            Command::Compile {
                config_url,
                output,
                format,
            } => Box::new(Compile::new(config_url, output, format)),
        }
    }
}

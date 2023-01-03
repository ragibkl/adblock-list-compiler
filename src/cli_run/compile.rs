use std::path::{Path, PathBuf};

use async_trait::async_trait;

use crate::compiler::AdblockCompiler;
use crate::config::{ConfigUrl, LoadConfig};
use crate::output::ZoneOutput;

use super::CliRun;

pub struct Compile {
    config_url: ConfigUrl,
    output: PathBuf,
    format: String,
}

impl Compile {
    pub fn new(config_url: &ConfigUrl, output: &Path, format: &str) -> Self {
        Self {
            config_url: config_url.to_owned(),
            output: output.to_owned(),
            format: format.to_owned(),
        }
    }
}

#[async_trait]
impl CliRun for Compile {
    async fn run(&self) -> u8 {
        println!("loading config:");
        println!("    config url: {}", &self.config_url);
        let load_config = LoadConfig::from(&self.config_url);
        let config = match load_config.load().await {
            Ok(c) => c,
            Err(e) => {
                println!("Failed to load config: {}", e);
                return 1;
            }
        };
        println!("loading config: done!");

        println!("compiling adblock list...");
        let adblock_compiler = match AdblockCompiler::init(&config) {
            Ok(ac) => ac,
            Err(e) => {
                println!("Failed to to init adblock compilerL {}", e);
                return 1;
            }
        };
        let adblock = adblock_compiler.compile().await;
        println!("compiling adblock list... done!");

        println!("writing output file:");
        println!("    output file: {}", &self.output.display());
        println!("    output format: {}", &self.format);
        let zone_output = ZoneOutput::new(adblock);
        zone_output.write_all(&self.output).unwrap();
        println!("writing output file: done!");

        0
    }
}

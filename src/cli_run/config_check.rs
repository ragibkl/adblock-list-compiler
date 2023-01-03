use async_trait::async_trait;

use crate::compiler::AdblockCompiler;
use crate::config::{ConfigUrl, LoadConfig};

use super::CliRun;

pub struct ConfigCheck {
    config_url: ConfigUrl,
}

impl ConfigCheck {
    pub fn new(config_url: &ConfigUrl) -> Self {
        Self {
            config_url: config_url.to_owned(),
        }
    }
}

#[async_trait]
impl CliRun for ConfigCheck {
    async fn run(&self) -> u8 {
        println!("config file: {}", &self.config_url);

        println!("loading config:");
        println!("    config url: {}", &self.config_url);
        let load_config = LoadConfig::from(&self.config_url);
        let config = match load_config.load().await {
            Ok(c) => c,
            Err(e) => {
                print!("Failed to load config: {}", e);
                return 1;
            }
        };
        println!("loading config: done!");

        println!("configuration:");
        println!("{:#?}", config);

        let adblock_compiler = match AdblockCompiler::init(&config) {
            Ok(ac) => ac,
            Err(e) => {
                println!("Failed to to init adblock compilerL {}", e);
                return 1;
            }
        };
        println!("Compiler Setting: {:#?}", &adblock_compiler);

        0
    }
}

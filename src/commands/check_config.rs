use std::path::Path;

use crate::compiler::AdblockCompiler;
use crate::config::{ConfigUrl, LoadConfig};

pub async fn check_config(config_url: &ConfigUrl, output: &Path, format: &str) {
    println!("config file: {}", config_url);
    println!("output file: {}", output.display());
    println!("output format: {}", format);

    println!("loading config:");
    println!("    config url: {}", config_url);
    let load_config = LoadConfig::from(config_url);
    let config = load_config.load().await.unwrap();
    println!("loading config: done!");

    println!("configuration:");
    println!("{:#?}", config);

    let adblock_compiler = AdblockCompiler::new(&config, config_url);
    println!("Compiler Setting: {:#?}", &adblock_compiler);
}

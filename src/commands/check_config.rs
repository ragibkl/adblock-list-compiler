use std::path::Path;

use crate::cli::ConfigUrl;
use crate::compiler::AdblockCompiler;
use crate::config::ConfigProvider;

pub async fn check_config(config_url: &ConfigUrl, output: &Path, format: &str) {
    println!("config file: {}", config_url);
    println!("output file: {}", output.display());
    println!("output format: {}", format);

    println!("loading config:");
    println!("    config url: {}", config_url);
    let config_provider = ConfigProvider::from(config_url);
    let source_config = config_provider.load().await.unwrap();
    println!("loading config: done!");

    println!("configuration:");
    println!("{:#?}", source_config);

    let adblock_compiler = AdblockCompiler::new(&source_config, config_url);
    println!("Compiler Setting: {:#?}", &adblock_compiler);
}

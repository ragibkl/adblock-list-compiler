use std::path::PathBuf;

use crate::cli::ConfigUrl;
use crate::compiler::AdblockCompiler;
use crate::config::provider::SourceConfigProvider;

pub async fn check_config(config_url: &ConfigUrl, output: &PathBuf, format: &str) {
    let conf_provider: SourceConfigProvider = match config_url {
        ConfigUrl::Url(url) => SourceConfigProvider::from(url),
        ConfigUrl::File(path) => SourceConfigProvider::from(path),
    };

    println!("configuration file: {}", config_url);
    println!("output file: {}", output.display());
    println!("output format: {}", format);

    println!("loading source config...");
    let source_config = conf_provider.load_config().await.unwrap();
    println!("loading source config... done!");

    println!("source configuration:");
    println!("{:#?}", source_config);

    let adblock_compiler = AdblockCompiler::new(&source_config, config_url);
    println!("Compiler Setting: {:#?}", &adblock_compiler);
}

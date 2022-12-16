use std::path::PathBuf;

use crate::cli::Config;
use crate::service::compiler::AdblockCompiler;
use crate::source_config::provider::SourceConfigProvider;

pub async fn compile(config: &Config, output: &PathBuf, format: &str) {
    let conf_provider: SourceConfigProvider = match config {
        Config::Url(url) => SourceConfigProvider::from(url),
        Config::File(path) => SourceConfigProvider::from(path),
    };

    println!("configuration file: {}", config);
    println!("output file: {}", output.display());
    println!("output format: {}", format);

    println!("loading source config...");
    let source_config = conf_provider.load_config().await.unwrap();
    println!("loading source config... done!");

    let adblock_compiler = AdblockCompiler::new(&source_config, config);

    adblock_compiler.compile().await;
}

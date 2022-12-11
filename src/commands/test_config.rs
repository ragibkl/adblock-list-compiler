use std::path::PathBuf;

use crate::cli::Config;
use crate::source_config::provider::SourceConfigProvider;

pub async fn test_config(config: &Config, output: &PathBuf, format: &str) {
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

    println!("source configuration:");
    println!("{:#?}", source_config);
}

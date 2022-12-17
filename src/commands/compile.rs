use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::cli::ConfigUrl;
use crate::compiler::AdblockCompiler;
use crate::output::ZoneOutput;
use crate::source_config::provider::SourceConfigProvider;

pub async fn compile(config_url: &ConfigUrl, output: &PathBuf, format: &str) {
    let conf_provider: SourceConfigProvider = match config_url {
        ConfigUrl::Url(url) => SourceConfigProvider::from(url),
        ConfigUrl::File(path) => SourceConfigProvider::from(path),
    };

    println!("loading source config from: {}", config_url);
    println!("loading source config...");
    let source_config = conf_provider.load_config().await.unwrap();
    println!("loading source config... done!");

    let adblock_compiler = AdblockCompiler::new(&source_config, config_url);

    println!("compiling adblock list...");
    let adblock = adblock_compiler.compile().await;
    println!("compiling adblock list... done!");

    println!(
        "Output will be written to {} using {} format.",
        output.display(),
        format
    );
    println!("writing output file...");
    let zone_output = ZoneOutput::new(adblock);
    let mut f = File::create(output).unwrap();
    f.write_all(zone_output.to_string().as_bytes()).unwrap();
    f.sync_all().unwrap();
    println!("writing output file... done!");
}

use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use crate::compiler::AdblockCompiler;
use crate::config::{ConfigUrl, LoadConfig};
use crate::output::ZoneOutput;

pub async fn compile(config_url: &ConfigUrl, output: &PathBuf, format: &str) {
    println!("loading config:");
    println!("    config url: {}", config_url);
    let load_config = LoadConfig::from(config_url);
    let config = load_config.load().await.unwrap();
    println!("loading config: done!");

    println!("compiling adblock list...");
    let adblock_compiler = AdblockCompiler::new(&config, config_url);
    let adblock = adblock_compiler.compile().await;
    println!("compiling adblock list... done!");

    println!("writing output file:");
    println!("    output file: {}", output.display());
    println!("    output format: {}", format);
    let zone_output = ZoneOutput::new(adblock);
    let mut f = File::create(output).unwrap();
    f.write_all(zone_output.to_string().as_bytes()).unwrap();
    f.sync_all().unwrap();
    println!("writing output file: done!");
}

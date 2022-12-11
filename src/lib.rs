pub mod cli;
mod commands;
mod source_config;

pub fn hello() {
    println!("Hello, world!");
}

pub async fn run() {
    let cli_args = cli::Cli::from_args();

    println!("{:#?}", &cli_args);

    match &cli_args.command {
        cli::Command::TestConfig {
            config,
            output,
            format,
        } => {
            commands::test_config::test_config(config, output, format).await;
        }
    };
}

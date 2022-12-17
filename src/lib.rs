pub mod cli;
mod commands;
mod compiler;
mod source_config;

pub fn hello() {
    println!("Hello, world!");
}

pub async fn run() {
    let cli_args = cli::Cli::from_args();

    match &cli_args.command {
        cli::Command::CheckConfig {
            config_url,
            output,
            format,
        } => {
            commands::check_config::check_config(config_url, output, format).await;
        }
        cli::Command::Compile {
            config_url,
            output,
            format,
        } => {
            commands::compile::compile(config_url, output, format).await;
        }
    };
}

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
            config,
            output,
            format,
        } => {
            commands::check_config::check_config(config, output, format).await;
        }
        cli::Command::Compile {
            config,
            output,
            format,
        } => {
            commands::compile::compile(config, output, format).await;
        }
    };
}

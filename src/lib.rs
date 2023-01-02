mod cli;
mod cli_run;
mod compiler;
mod config;
mod fetch;
mod output;

pub fn hello() {
    println!("Hello, world!");
}

pub async fn run() -> u8 {
    let cli_args = cli::Cli::from_args();
    let cmd = cli_args.into_cli_run();

    cmd.run().await
}

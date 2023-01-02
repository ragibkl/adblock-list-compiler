use std::process::ExitCode;

use adblock_list_compiler::run;

#[tokio::main]
async fn main() -> ExitCode {
    let exit_code = run().await;

    ExitCode::from(exit_code)
}

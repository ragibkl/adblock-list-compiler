use adblock_list_compiler::cli::Cli;
use adblock_list_compiler::hello;

fn main() {
    hello();

    let cli = Cli::from_args();

    println!("{:#?}", cli);
}

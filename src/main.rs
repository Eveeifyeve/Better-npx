use clap::{Parser, Subcommand};
mod cmd;

#[derive(Parser)] // requires `derive` feature
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand)]
enum SubCommands {
    Create(cmd::Create),
}

fn main() {
    let opts: Cli = Cli::parse();

    match opts.subcommand {
        SubCommands::Create(c) => {
            println!("Sub Command: Create Called. {}", c.package_name);
        },
    }
}
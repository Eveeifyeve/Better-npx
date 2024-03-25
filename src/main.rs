use clap::{Parser, Subcommand};
mod cmd;
mod scripts;

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
           let pkg_bins = scripts::check_pkg_managers();

           println!("{:?}", pkg_bins);
        },
    }
}
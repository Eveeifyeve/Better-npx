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
           let mut package_name = "";
           match c.package_name.as_str() {
            "astro" => {
                package_name = "astro-latest";
            }
            _ => {
                package_name = &c.package_name;
            }
           }
           let pkg_bins = scripts::check_pkg_managers();
           println!("PKG Name: {:?}", package_name);

           println!("{:?}", pkg_bins);
        },
    }
}
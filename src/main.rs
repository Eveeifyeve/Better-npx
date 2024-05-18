use clap::{Parser, Subcommand};
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor},
};
use std::io::stdout;
use std::error::Error;
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

fn main() -> Result<(), Box<dyn Error>> {
    execute!(
        stdout(),
        SetBackgroundColor(Color::Yellow),
        Print("\n\n\
        -----------------------------------------\n\
        |               NOTICE                   |\n\
        |                                        |\n\
        |  This project, Better-npx, is now      |\n\
        |  archived.                             |\n\
        |  Development has ceased,               |\n\
        |  Check the readme for more info.       |\n\
        |                                        |\n\
        -----------------------------------------"),
        ResetColor
    )?;

    println!("\n\n");

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

    Ok(())
}
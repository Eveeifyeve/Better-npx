use clap::Parser;

#[derive(Parser)]
pub struct Create {
    pub package_name: String,
}
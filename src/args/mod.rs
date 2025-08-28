use clap::{Parser, Subcommand};

pub mod dps;
pub mod config;

#[derive(Parser, Debug)]
#[command(name="EnderCliTools", author="Endkind Ender", version, about="EnderCliTools is a lightweight collection of CLI utilities that make working in the terminal faster and more convenient.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Dps(dps::DpsArgs),
    Config(config::ConfigArgs),
}

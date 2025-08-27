use clap::{Parser, Subcommand};

pub mod dps;

#[derive(Parser, Debug)]
#[command(name="EnderCliTools", author="Endkind Ender", version, about="Lorem Ipsum")]
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
}

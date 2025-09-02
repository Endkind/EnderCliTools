use crate::args::config::Normalizable;
use clap::{Args, Subcommand};
use serde::Serialize;

#[derive(Args, Debug)]
pub struct GetArgs {
    #[command(subcommand)]
    pub command: Option<GetCommands>,
}

#[derive(Subcommand, Debug)]
pub enum GetCommands {
    Table(GetTableArgs),
    Dps(GetDpsArgs),
}

#[derive(Args, Debug, Clone, Serialize, Default)]
pub struct GetTableArgs {
    #[arg(short, long)]
    pub all: bool,
    #[arg(long)]
    pub preset: bool,
    #[arg(long)]
    pub modifier: bool,
}

impl Normalizable for GetTableArgs {
    fn set_all(&mut self, value: bool) {
        self.all = value;
    }
}

#[derive(Args, Debug, Clone, Serialize, Default)]
pub struct GetDpsArgs {
    #[arg(short, long)]
    pub all: bool,
    #[arg(long)]
    pub headers: bool,
}

impl Normalizable for GetDpsArgs {
    fn set_all(&mut self, value: bool) {
        self.all = value;
    }
}

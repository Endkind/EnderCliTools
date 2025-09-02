use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
use crate::args::config::Normalizable;

#[derive(Args, Debug)]
pub struct ResetArgs {
    #[command(subcommand)]
    pub command: Option<ResetCommands>,
}

#[derive(Subcommand, Debug)]
pub enum ResetCommands {
    Table(ResetTableArgs),
    Dps(ResetDpsArgs),
}

#[derive(Args, Debug, Clone, Serialize, Deserialize)]
pub struct ResetTableArgs {
    #[arg(short, long)]
    pub all: bool,
    #[arg(long)]
    pub preset: bool,
    #[arg(long)]
    pub modifier: bool,
}

impl Normalizable for ResetTableArgs {
    fn set_all(&mut self, value: bool) {
        self.all = value;
    }
}

#[derive(Args, Debug, Clone, Serialize, Deserialize)]
pub struct ResetDpsArgs {
    #[arg(short, long)]
    pub all: bool,
    #[arg(long)]
    pub headers: bool,
}

impl Normalizable for ResetDpsArgs {
    fn set_all(&mut self, value: bool) {
        self.all = value;
    }
}

use crate::config::model::dps::DpsHeader;
use crate::config::model::table::{TableModifiers, TablePresets};
use clap::{Args, Subcommand};

#[derive(Args, Debug, Clone)]
pub struct SetArgs {
    #[command(subcommand)]
    pub command: SetCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SetCommands {
    Table(SetTableArgs),
    Dps(SetDpsArgs),
}

#[derive(Args, Debug, Clone, Copy)]
pub struct SetTableArgs {
    #[arg(long, value_enum)]
    pub preset: Option<TablePresets>,
    #[arg(long, value_enum)]
    pub modifier: Option<TableModifiers>,
}

#[derive(Args, Debug, Clone)]
pub struct SetDpsArgs {
    #[arg(long, value_enum)]
    pub headers: Option<Vec<DpsHeader>>,
}

use clap::{Args, Subcommand};
use crate::config::model::dps::DpsHeader;
use crate::config::model::table::{TableModifiers, TablePresets};

#[derive(Args, Debug)]
pub struct SetArgs {
    #[command(subcommand)]
    pub command: SetCommands,
}

#[derive(Subcommand, Debug)]
pub enum SetCommands {
    Table(SetTableArgs),
    Dps(SetDpsArgs),
}

#[derive(Args, Debug)]
pub struct SetTableArgs {
    #[arg(long, value_enum)]
    pub preset: Option<TablePresets>,
    #[arg(long, value_enum)]
    pub modifier: Option<TableModifiers>,
}

#[derive(Args, Debug)]
pub struct SetDpsArgs {
    #[arg(long, value_enum)]
    pub headers: Option<Vec<DpsHeader>>,
}

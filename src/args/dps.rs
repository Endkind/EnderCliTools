use clap::Args;
use crate::config::model::dps::DpsHeader;
use crate::config::model::table::{TablePresets, TableModifiers};

#[derive(Args, Debug)]
/// Pretty replacement for `docker ps`
pub struct DpsArgs {
    /// Show all containers (like `docker ps -a`)
    #[arg(short, long)]
    pub all: bool,
    #[arg(long, value_enum)]
    pub table_preset: Option<TablePresets>,
    #[arg(long, value_enum)]
    pub table_modifier: Option<TableModifiers>,
    #[arg(long, value_enum)]
    pub headers: Option<Vec<DpsHeader>>,
}

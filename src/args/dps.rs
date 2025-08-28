use clap::Args;
use crate::config::model;

#[derive(Args, Debug)]
/// Pretty replacement for `docker ps`
pub struct DpsArgs {
    /// Show all containers (like `docker ps -a`)
    #[arg(short, long)]
    pub all: bool,
    #[arg(long, value_enum)]
    pub table_preset: Option<model::TablePresets>,
    #[arg(long, value_enum)]
    pub table_modifier: Option<model::TableModifiers>,
}

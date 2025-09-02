use crate::config::model::dps::DpsHeader;
use crate::config::model::table::{TableModifiers, TablePresets};
use clap::Args;

#[derive(Args, Debug)]
/// Pretty replacement for `docker ps`
pub struct DpsArgs {
    /// Show all containers (default shows just running)
    #[arg(short, long)]
    pub all: bool,
    /// Filter output based on conditions provided
    #[arg(short, long)]
    pub filter: Option<String>,
    /// Show n last created containers (includes all states)
    #[arg(short='n', long, default_value_t = -1)]
    pub last: i32,
    /// Show the latest created container (includes all states)
    #[arg(short, long)]
    pub latest: bool,
    /// Don't truncate output
    #[arg(long)]
    pub no_trunc: bool,
    /// Only display container IDs
    #[arg(short, long)]
    pub quiet: bool,
    /// Display total file sizes
    #[arg(short, long)]
    pub size: bool,
    #[arg(long, value_enum)]
    pub table_preset: Option<TablePresets>,
    #[arg(long, value_enum)]
    pub table_modifier: Option<TableModifiers>,
    #[arg(long, value_enum)]
    pub headers: Option<Vec<DpsHeader>>,
    #[arg(long, value_enum)]
    pub add_headers: Option<Vec<DpsHeader>>,
}

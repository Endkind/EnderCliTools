use crate::config::model::dcps::DcpsHeader;
use crate::config::model::table::{TableModifiers, TablePresets};
use clap::{Args, ValueEnum};
use std::fmt;

#[derive(Args, Debug, Clone)]
/// Pretty replacement for `docker compose ps`
pub struct DcpsArgs {
    /// Show all containers (default shows just running)
    #[arg(short, long)]
    pub all: bool,
    /// Filter output based on conditions provided
    #[arg(short, long)]
    pub filter: Option<String>,
    /// Don't truncate output
    #[arg(long)]
    pub no_trunc: bool,
    /// Exclude orphaned services (not declared by project)
    #[arg(long)]
    pub no_orphans: bool,
    /// Only display container IDs
    #[arg(short, long)]
    pub quiet: bool,
    /// Display services
    #[arg(long)]
    pub services: bool,
    /// Filter services by status.
    #[arg(long)]
    pub status: Option<Vec<Status>>,
    #[arg(long, value_enum)]
    pub table_preset: Option<TablePresets>,
    #[arg(long, value_enum)]
    pub table_modifier: Option<TableModifiers>,
    #[arg(long, value_enum)]
    pub headers: Option<Vec<DcpsHeader>>,
    #[arg(long, value_enum)]
    pub add_headers: Option<Vec<DcpsHeader>>,
}

#[derive(Debug, Clone, ValueEnum, Copy)]
pub enum Status {
    Paused,
    Restarting,
    Removing,
    Running,
    Dead,
    Created,
    Exited,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("{:?}", self).to_ascii_lowercase();
        write!(f, "{}", s)
    }
}

use dps::DpsConfig;
use serde::{Deserialize, Serialize};
use table::TableConfig;

pub mod dps;
pub mod table;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub table: TableConfig,
    pub dps: DpsConfig,
}

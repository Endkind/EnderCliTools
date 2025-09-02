use serde::{Deserialize, Serialize};
use table::TableConfig;
use dps::DpsConfig;

pub mod table;
pub mod dps;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub table: TableConfig,
    pub dps: DpsConfig,
}

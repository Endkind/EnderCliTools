use crate::config::model::dcps::DcpsConfig;
use dps::DpsConfig;
use serde::{Deserialize, Serialize};
use table::TableConfig;

pub mod dcps;
pub mod dps;
pub mod table;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub table: TableConfig,
    pub dps: DpsConfig,
    pub dcps: DcpsConfig,
}

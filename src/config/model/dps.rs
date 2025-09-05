use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DpsConfig {
    pub headers: Vec<DpsHeader>,
}

impl Default for DpsConfig {
    fn default() -> DpsConfig {
        DpsConfig {
            headers: vec![
                DpsHeader::Id,
                DpsHeader::Names,
                DpsHeader::Image,
                DpsHeader::Status,
                DpsHeader::Ports,
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ValueEnum)]
pub enum DpsHeader {
    Id,
    Names,
    Image,
    Status,
    Ports,
    Command,
    CreatedAt,
    Created,
    Size,
    Labels,
    Mounts,
}

impl fmt::Display for DpsHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DpsHeader {
    pub fn display_name(&self) -> &str {
        match self {
            DpsHeader::Id => "ID",
            DpsHeader::Names => "Names",
            DpsHeader::Image => "Image",
            DpsHeader::Status => "Status",
            DpsHeader::Ports => "Ports",
            DpsHeader::Command => "Command",
            DpsHeader::CreatedAt => "CreatedAt",
            DpsHeader::Created => "RunningFor",
            DpsHeader::Size => "Size",
            DpsHeader::Labels => "Labels",
            DpsHeader::Mounts => "Mounts",
        }
    }
}

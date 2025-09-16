use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct DcpsConfig {
    pub headers: Vec<DcpsHeader>,
}

impl Default for DcpsConfig {
    fn default() -> DcpsConfig {
        DcpsConfig {
            headers: vec![
                DcpsHeader::Service,
                DcpsHeader::Image,
                DcpsHeader::Status,
                DcpsHeader::Ports,
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, ValueEnum)]
pub enum DcpsHeader {
    Id,
    Service,
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

impl fmt::Display for DcpsHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl DcpsHeader {
    pub fn display_name(&self) -> &str {
        match self {
            DcpsHeader::Id => "ID",
            DcpsHeader::Service => "Service",
            DcpsHeader::Names => "Names",
            DcpsHeader::Image => "Image",
            DcpsHeader::Status => "Status",
            DcpsHeader::Ports => "Ports",
            DcpsHeader::Command => "Command",
            DcpsHeader::CreatedAt => "CreatedAt",
            DcpsHeader::Created => "RunningFor",
            DcpsHeader::Size => "Size",
            DcpsHeader::Labels => "Labels",
            DcpsHeader::Mounts => "Mounts",
        }
    }
}

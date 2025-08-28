use std::fmt;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use comfy_table::{presets, modifiers};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    pub table: TableConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TableConfig {
    pub preset: TablePresets,
    pub modifier: TableModifiers,
}

#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub enum TablePresets {
    ASCII_FULL,
    ASCII_FULL_CONDENSED,
    ASCII_NO_BORDERS,
    ASCII_BORDERS_ONLY,
    ASCII_BORDERS_ONLY_CONDENSED,
    ASCII_HORIZONTAL_ONLY,
    ASCII_MARKDOWN,
    UTF8_FULL,
    UTF8_FULL_CONDENSED,
    UTF8_NO_BORDERS,
    UTF8_BORDERS_ONLY,
    UTF8_HORIZONTAL_ONLY,
    NOTHING,
}

impl fmt::Display for TablePresets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for TablePresets {
    fn default() -> Self {
        TablePresets::UTF8_FULL
    }
}

impl TablePresets {
    pub fn to_preset(&self) -> &str {
        match self {
            TablePresets::ASCII_FULL => presets::ASCII_FULL,
            TablePresets::ASCII_FULL_CONDENSED => presets::ASCII_FULL_CONDENSED,
            TablePresets::ASCII_NO_BORDERS => presets::ASCII_NO_BORDERS,
            TablePresets::ASCII_BORDERS_ONLY => presets::ASCII_BORDERS_ONLY,
            TablePresets::ASCII_BORDERS_ONLY_CONDENSED  => presets::ASCII_BORDERS_ONLY_CONDENSED,
            TablePresets::ASCII_HORIZONTAL_ONLY => presets::ASCII_HORIZONTAL_ONLY,
            TablePresets::ASCII_MARKDOWN => presets::ASCII_MARKDOWN,
            TablePresets::UTF8_FULL => presets::UTF8_FULL,
            TablePresets::UTF8_FULL_CONDENSED => presets::UTF8_FULL_CONDENSED,
            TablePresets::UTF8_NO_BORDERS => presets::UTF8_NO_BORDERS,
            TablePresets::UTF8_BORDERS_ONLY => presets::UTF8_BORDERS_ONLY,
            TablePresets::UTF8_HORIZONTAL_ONLY => presets::UTF8_HORIZONTAL_ONLY,
            TablePresets::NOTHING => presets::NOTHING,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum)]
pub enum TableModifiers {
    UTF8_ROUND_CORNERS,
    UTF8_SOLID_INNER_BORDERS
}

impl fmt::Display for TableModifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for TableModifiers {
    fn default() -> Self {
        TableModifiers::UTF8_ROUND_CORNERS
    }
}

impl TableModifiers {
    pub fn to_modifier(&self) -> &str {
        match self {
            TableModifiers::UTF8_ROUND_CORNERS => modifiers::UTF8_ROUND_CORNERS,
            TableModifiers::UTF8_SOLID_INNER_BORDERS => modifiers::UTF8_SOLID_INNER_BORDERS,
        }
    }
}

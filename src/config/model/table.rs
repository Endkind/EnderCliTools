use clap::ValueEnum;
use comfy_table::{modifiers, presets};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TableConfig {
    pub preset: TablePresets,
    pub modifier: TableModifiers,
}

#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum, Default)]
pub enum TablePresets {
    AsciiFull,
    AsciiFullCondensed,
    AsciiNoBorders,
    AsciiBordersOnly,
    AsciiBordersOnlyCondensed,
    AsciiHorizontalOnly,
    AsciiMarkdown,
    #[default]
    Utf8Full,
    Utf8FullCondensed,
    Utf8NoBorders,
    Utf8BordersOnly,
    Utf8HorizontalOnly,
    Nothing,
}

impl fmt::Display for TablePresets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TablePresets {
    pub fn to_preset(&self) -> &str {
        match self {
            TablePresets::AsciiFull => presets::ASCII_FULL,
            TablePresets::AsciiFullCondensed => presets::ASCII_FULL_CONDENSED,
            TablePresets::AsciiNoBorders => presets::ASCII_NO_BORDERS,
            TablePresets::AsciiBordersOnly => presets::ASCII_BORDERS_ONLY,
            TablePresets::AsciiBordersOnlyCondensed => presets::ASCII_BORDERS_ONLY_CONDENSED,
            TablePresets::AsciiHorizontalOnly => presets::ASCII_HORIZONTAL_ONLY,
            TablePresets::AsciiMarkdown => presets::ASCII_MARKDOWN,
            TablePresets::Utf8Full => presets::UTF8_FULL,
            TablePresets::Utf8FullCondensed => presets::UTF8_FULL_CONDENSED,
            TablePresets::Utf8NoBorders => presets::UTF8_NO_BORDERS,
            TablePresets::Utf8BordersOnly => presets::UTF8_BORDERS_ONLY,
            TablePresets::Utf8HorizontalOnly => presets::UTF8_HORIZONTAL_ONLY,
            TablePresets::Nothing => presets::NOTHING,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ValueEnum, Default)]
pub enum TableModifiers {
    #[default]
    Utf8RoundCorners,
    Utf8SolidInnerBorders,
}

impl fmt::Display for TableModifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TableModifiers {
    pub fn to_modifier(&self) -> &str {
        match self {
            TableModifiers::Utf8RoundCorners => modifiers::UTF8_ROUND_CORNERS,
            TableModifiers::Utf8SolidInnerBorders => modifiers::UTF8_SOLID_INNER_BORDERS,
        }
    }
}

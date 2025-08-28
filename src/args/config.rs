use clap::{Args, Subcommand};
use serde::Serialize;
use toml::Value;
use crate::config::model;

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Set(SetConfigArgs),
    Get(GetConfigArgs),
    Reset(ResetConfigArgs),
}

#[derive(Args, Debug)]
pub struct SetConfigArgs {
    #[arg(long, value_enum)]
    pub table_preset: Option<model::TablePresets>,
    #[arg(long, value_enum)]
    pub table_modifier: Option<model::TableModifiers>,
}

#[derive(Args, Debug, Clone, Serialize)]
pub struct GetConfigArgs {
    #[arg(short, long)]
    pub all: bool,
    #[arg(long)]
    pub table_preset: bool,
    #[arg(long)]
    pub table_modifier: bool,
}

impl GetConfigArgs {
    pub fn mormalize(mut self) -> Self {
        let v = Value::try_from(&self).expect("serialize GetConfigArgs to TOML");
        let mut any_other = false;

        if let Some(tbl) = v.as_table() {
            for (k, vv) in tbl {
                if k == "all" {
                    continue;
                }
                if let Value::Boolean(b) = vv {
                    if *b {
                        any_other = true;
                        break;
                    }
                }
            }
        }

        if !any_other {
            self.all = true;
        }
        self
    }
}

#[derive(Args, Debug)]
pub struct ResetConfigArgs {
    #[arg(short, long)]
    pub all: bool,
    #[arg(long)]
    pub table_preset: bool,
    #[arg(long)]
    pub table_modifier: bool,
}

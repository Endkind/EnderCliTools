use clap::{Args, Subcommand};
use serde::Serialize;
use toml::Value;
use set::SetArgs;
use get::GetArgs;
use reset::ResetArgs;

pub mod set;
pub mod get;
pub mod reset;

pub trait Normalizable: Serialize + Sized {
    fn set_all(&mut self, value: bool);

    fn normalize(mut self) -> Self {
        let v = Value::try_from(&self).expect("serialize to TOML");

        if let Some(tbl) = v.as_table() {
            let any_other = tbl.iter().any(|(k, vv)| {
                k != "all" && matches!(vv, Value::Boolean(true))
            });

            if !any_other {
                self.set_all(true);
            }
        }

        self
    }
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug)]
pub enum ConfigCommands {
    Set(SetArgs),
    Get(GetArgs),
    Reset(ResetArgs),
}

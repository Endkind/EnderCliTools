use clap::{Args, Subcommand};
use get::GetArgs;
use reset::ResetArgs;
use serde::Serialize;
use set::SetArgs;
use toml::Value;

pub mod get;
pub mod reset;
pub mod set;

pub trait Normalizable: Serialize + Sized {
    fn set_all(&mut self, value: bool);

    fn normalize(mut self) -> Self {
        let v = Value::try_from(&self).expect("serialize to TOML");

        if let Some(tbl) = v.as_table() {
            let any_other = tbl
                .iter()
                .any(|(k, vv)| k != "all" && matches!(vv, Value::Boolean(true)));

            if !any_other {
                self.set_all(true);
            }
        }

        self
    }
}

#[derive(Args, Debug, Clone)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: ConfigCommands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ConfigCommands {
    Set(SetArgs),
    Get(GetArgs),
    Reset(ResetArgs),
}

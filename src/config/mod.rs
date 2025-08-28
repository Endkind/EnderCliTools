use std::fs;
use std::path::PathBuf;
use anyhow::{Context, Result};
use directories::ProjectDirs;

pub mod model;
pub use model::Config;

impl Config {
    pub fn load() -> Result<Self> {
        let proj_dir = ProjectDirs::from("net", "endkind", "enderclitools")
            .context("Could not determine config directory")?;
        let path: PathBuf = proj_dir.config_dir().join("config.toml");

        if path.exists() {
            let data = fs::read_to_string(path)?;
            let cfg: Config = toml::from_str(&data)?;
            Ok(cfg)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let proj_dir = ProjectDirs::from("net", "endkind", "enderclitools")
            .context("Could not determine config directory")?;
        let path: PathBuf = proj_dir.config_dir().join("config.toml");

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = toml::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }
}

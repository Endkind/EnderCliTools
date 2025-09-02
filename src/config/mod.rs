use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::{Context, Result};
use directories::ProjectDirs;

pub mod model;
pub use model::Config;

static CONFIG_ERROR_REPORTED: AtomicBool = AtomicBool::new(false);

impl Config {
    fn config_file_path() -> Result<PathBuf> {
        let proj_dir = ProjectDirs::from("net", "endkind", "enderclitools")
            .context("Could not determine config directory")?;
        Ok(proj_dir.config_dir().join("config.toml"))
    }

    fn print_config_error_once() {
        if !CONFIG_ERROR_REPORTED.swap(true, Ordering::SeqCst) {
            eprintln!(
                "Failed to load config. Falling back to defaults.\n\
            Run 'ect config reset' to regenerate your config.toml"
            );
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_file_path()?;

        if path.exists() {
            let data = fs::read_to_string(path)?;
            match toml::from_str::<Config>(&data) {
                Ok(cfg) => Ok(cfg),
                Err(_) => {
                    Self::print_config_error_once();
                    Ok(Config::default())
                }
            }
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_file_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let data = toml::to_string_pretty(self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn backup() -> Result<Option<String>> {
        let path = Self::config_file_path()?;
        if !path.exists() {
            return Ok(None);
        }

        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let backup_path = path.with_extension(format!("toml.{}.bak", ts));

        fs::copy(&path, &backup_path)
            .with_context(|| format!("Failed to create backup at {}", backup_path.display()))?;

        Ok(Some(backup_path.to_string_lossy().into_owned()))
    }
}

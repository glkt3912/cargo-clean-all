use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub paths: PathsConfig,
    pub cleanup: CleanupConfig,
    pub logging: LoggingConfig,
    pub notification: NotificationConfig,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PathsConfig {
    pub scan_roots: Vec<String>,
    pub exclude_dirs: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CleanupConfig {
    pub target_only: bool,
    pub min_size_mb: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoggingConfig {
    pub log_file: String,
    pub level: String,
    pub max_files: usize,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub title: String,
    pub error_only: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            paths: PathsConfig {
                scan_roots: vec!["/Volumes/Dev-SSD/dev".to_string()],
                exclude_dirs: vec!["node_modules".to_string(), ".git".to_string()],
            },
            cleanup: CleanupConfig {
                target_only: true,
                min_size_mb: 10,
            },
            logging: LoggingConfig {
                log_file: "~/.local/share/cargo-clean-all/clean.log".to_string(),
                level: "info".to_string(),
                max_files: 10,
            },
            notification: NotificationConfig {
                enabled: true,
                title: "Cargo Clean All".to_string(),
                error_only: false,
            },
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = Self::config_path();
        if !config_path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(config_path)?;
        Ok(toml::from_str(&content)?)
    }

    fn config_path() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        PathBuf::from(home).join(".config/cargo-clean-all/config.toml")
    }
}

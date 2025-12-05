use serde::{Deserialize, Serialize};
use dirs::config_dir;
use std::fs::{create_dir_all, read_to_string, write};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub default_base: String,
    pub default_output: String,
    pub cache_max_age_minutes: i64,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            default_base: "USD".to_string(),
            default_output: "plain".to_string(),
            cache_max_age_minutes: 120,
        }
    }
}

pub struct Config {
    path: PathBuf,
}

#[allow(dead_code)]
impl Config {
    pub fn new() -> Self {
        let root = config_dir().unwrap_or_else(|| PathBuf::from(".")).join("currency-cli");
        let path = root.join("config.toml");
        let _ = create_dir_all(&root);
        Self { path }
    }

    pub fn load(&self) -> AppConfig {
        match read_to_string(&self.path) {
            Ok(s) => toml::from_str(&s).unwrap_or_default(),
            Err(_) => AppConfig::default(),
        }
    }

    #[allow(dead_code)]
    pub fn save(&self, cfg: &AppConfig) {
        if let Ok(s) = toml::to_string(cfg) {
            let _ = write(&self.path, s);
        }
    }
}
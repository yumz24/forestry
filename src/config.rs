use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub default_preview: Option<bool>,
    pub log_level: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        let config_path = PathBuf::from(".forestory.toml");
        if let Ok(content) = fs::read_to_string(config_path) {
            return toml::from_str(&content).unwrap_or(Self::default());
        }
        Self::default()
    }

    fn default() -> Self {
        Self {
            default_preview: Some(true),
            log_level: Some("info".to_string()),
        }
    }
}

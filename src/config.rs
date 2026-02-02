use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    pub default_yes: Option<bool>,
    pub log_level: Option<String>,
}

impl Config {
    pub fn load(path: Option<String>) -> Self {
        // 引数で指定されたパス、またはデフォルトの .forestry.toml を探す
        let config_path = path.map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".forestry.toml"));

        if let Ok(content) = fs::read_to_string(config_path) {
            return toml::from_str(&content).unwrap_or_default();
        }
        Self::default()
    }

}

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::Local;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub default_yes: Option<bool>,
    pub log_level: Option<String>,
    pub templates: Option<HashMap<String, String>>,
    #[serde(default = "default_overwrite")]
    pub overwrite: bool,
    #[serde(default = "default_ignore")]
    pub ignore_patterns: Vec<String>,
    pub project_name: Option<String>,
}

fn default_overwrite() -> bool {
    false
}

fn default_ignore() -> Vec<String> {
    vec![".git".to_string(), "node_modules".to_string()]
}

impl Config {
    pub fn load(path: Option<String>) -> Self {
        let config_path = path
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(".forestry.toml"));

        if let Ok(content) = fs::read_to_string(config_path) {
            return toml::from_str(&content).unwrap_or_else(|_| Self::default());
        }
        Self::default()
    }

    pub fn resolve_template(&self, raw_content: &str) -> String {
        let date = Local::now().format("%Y-%m-%d").to_string();
        let project = self.project_name.as_deref().unwrap_or("Unknown Project");

        raw_content
            .replace("${DATE}", &date)
            .replace("${PROJECT_NAME}", project)
    }

    pub fn is_ignored(&self, path: &str) -> bool {
        self.ignore_patterns.iter().any(|pattern| path.contains(pattern))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_yes: Some(false),
            log_level: Some("info".to_string()),
            templates: None,
            overwrite: false,
            ignore_patterns: default_ignore(),
            project_name: None,
        }
    }
}

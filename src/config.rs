use std::fs;

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub icon_theme: Option<String>,
}

impl Config {
    pub fn load_from_file(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_default()
    }
}

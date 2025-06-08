use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub icon_theme: Option<String>,
}

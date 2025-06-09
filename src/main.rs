mod config;
mod message;
mod state;
mod ui;

use config::Config;
use iced::{Application, Settings};
use state::Dashboard;
use std::path::PathBuf;

fn main() -> iced::Result {
    let path = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("hyprdashboard/config.toml");

    let config = Config::load_from_file(path.to_str().unwrap_or("config.toml"));
    println!("▶ Loaded config: {:?}", config);
    Dashboard::run(Settings::with_flags(config))
}

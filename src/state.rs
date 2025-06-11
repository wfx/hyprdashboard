use crate::config::Config;
use crate::ui::{launcher_view, settings_view};
use iced::widget::{image, svg};
use iced::{Application, Command, Element, Theme};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use xdg::BaseDirectories;

#[derive(Debug)]
pub struct AppInfo {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
}

pub struct Dashboard {
    pub config: Config,
    pub apps: Vec<AppInfo>,
    pub show_settings: bool,
}

impl Dashboard {
    pub fn new(config: Config) -> Self {
        let apps = Self::find_applications(&config);
        Self {
            config,
            apps,
            show_settings: false,
        }
    }

    fn find_applications(config: &Config) -> Vec<AppInfo> {
        let mut apps = Vec::new();
        let xdg_dirs = BaseDirectories::new().unwrap();

        let mut paths: Vec<PathBuf> = Vec::new();
        paths.push(xdg_dirs.get_data_home());
        paths.extend(xdg_dirs.get_data_dirs());

        for base in paths {
            let dir = base.join("applications");
            if let Ok(entries) = dir.read_dir() {
                for entry in entries.flatten() {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        let name = content
                            .lines()
                            .find_map(|line| line.strip_prefix("Name="))
                            .unwrap_or("")
                            .to_string();
                        let exec = content
                            .lines()
                            .find_map(|line| line.strip_prefix("Exec="))
                            .unwrap_or("")
                            .to_string();
                        let icon_name = content
                            .lines()
                            .find_map(|line| line.strip_prefix("Icon="))
                            .unwrap_or("");

                        if icon_name.is_empty() {
                            continue;
                        }

                        let icon = resolve_icon(icon_name, config.icon_theme.as_deref());

                        if let Some(ref path) = icon {
                            log::info!("✓ Icon '{}' found: {}", icon_name, path);
                        } else {
                            log::error!("✗ Icon '{}' not found in any theme (searched with theme hint: {:?})", icon_name, config.icon_theme);
                        }

                        if !name.is_empty() && !exec.is_empty() {
                            apps.push(AppInfo { name, exec, icon });
                        }
                    }
                }
            }
        }

        apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        apps
    }
}

impl Application for Dashboard {
    type Executor = iced::executor::Default;
    type Message = crate::message::Message;
    type Theme = Theme;
    type Flags = Config;

    fn new(config: Config) -> (Self, Command<Self::Message>) {
        (Dashboard::new(config), Command::none())
    }

    fn title(&self) -> String {
        String::from("Hypr Dashboard")
    }

    fn theme(&self) -> Self::Theme {
        Theme::GruvboxLight
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            crate::message::Message::LaunchApp(cmd) => {
                if !cmd.is_empty() {
                    if let Err(e) = std::process::Command::new("sh").arg("-c").arg(&cmd).spawn() {
                        log::error!("failed to launch {}: {}", cmd, e);
                    }
                }
                Command::none()
            }
            crate::message::Message::ToggleSettings => {
                self.show_settings = !self.show_settings;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Self::Message> {
        if self.show_settings {
            settings_view()
        } else {
            launcher_view(&self.apps)
        }
    }
}

struct ThemeInfo {
    directories: Vec<String>,
    inherits: Vec<String>,
}

fn parse_index_theme<P: AsRef<Path>>(path: P) -> Option<ThemeInfo> {
    let content = fs::read_to_string(path).ok()?;
    let mut directories = Vec::new();
    let mut inherits = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("Directories=") {
            directories.extend(
                rest.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from),
            );
        }
        if let Some(rest) = line.strip_prefix("Inherits=") {
            inherits.extend(
                rest.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from),
            );
        }
    }

    Some(ThemeInfo {
        directories,
        inherits,
    })
}

static ICON_CACHE: Lazy<Mutex<HashMap<(String, String), Option<String>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

fn resolve_icon(name: &str, theme_hint: Option<&str>) -> Option<String> {
    let key = (name.to_string(), theme_hint.unwrap_or("").to_string());
    if let Some(result) = ICON_CACHE.lock().unwrap().get(&key) {
        return result.clone();
    }

    let exts = ["png", "svg", "xpm"];
    let mut themes = Vec::new();

    if let Some(theme_name) = theme_hint {
        themes.push(theme_name.to_string());
    }
    if !themes.contains(&"hicolor".to_string()) {
        themes.push("hicolor".into());
    }

    let theme_paths = [
        "/usr/share/icons",
        "/usr/local/share/icons",
        &format!("{}/.icons", env::var("HOME").unwrap_or_default()),
    ];

    while let Some(theme) = themes.pop() {
        for base in &theme_paths {
            let theme_dir = Path::new(base).join(&theme);
            let index_path = theme_dir.join("index.theme");
            if !index_path.exists() {
                continue;
            }
            if let Some(theme_info) = parse_index_theme(&index_path) {
                for dir in &theme_info.directories {
                    for ext in &exts {
                        let candidate = theme_dir.join(dir).join(format!("{}.{}", name, ext));
                        if candidate.exists() {
                            let result = Some(candidate.to_string_lossy().into_owned());
                            ICON_CACHE.lock().unwrap().insert(key, result.clone());
                            return result;
                        }
                    }
                }
                for ext in &exts {
                    let candidate = theme_dir
                        .join("scalable/apps")
                        .join(format!("{}.{}", name, ext));
                    if candidate.exists() {
                        let result = Some(candidate.to_string_lossy().into_owned());
                        ICON_CACHE.lock().unwrap().insert(key, result.clone());
                        return result;
                    }
                }
                for inherit in theme_info.inherits {
                    if !themes.contains(&inherit) {
                        themes.push(inherit);
                    }
                }
            }
        }
    }

    for base in &theme_paths {
        for ext in &exts {
            let candidate = Path::new(base)
                .join("pixmaps")
                .join(format!("{}.{}", name, ext));
            if candidate.exists() {
                let result = Some(candidate.to_string_lossy().into_owned());
                ICON_CACHE.lock().unwrap().insert(key, result.clone());
                return result;
            }
        }
    }

    for base in &theme_paths {
        for ext in &exts {
            let candidate = Path::new(base).join(format!("{}.{}", name, ext));
            if candidate.exists() {
                let result = Some(candidate.to_string_lossy().into_owned());
                ICON_CACHE.lock().unwrap().insert(key, result.clone());
                return result;
            }
        }
    }

    ICON_CACHE.lock().unwrap().insert(key, None);
    None
}

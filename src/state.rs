use crate::message::Message;
use crate::ui::{launcher_view, settings_view};
use iced::{Application, Command, Element, Theme};

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use ini::Ini;
use std::collections::HashSet;

#[derive(Debug)]
pub struct AppInfo {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
}

fn icon_base_dirs() -> Vec<PathBuf> {
    let mut dirs_vec = Vec::new();
    if let Some(dir) = std::env::var_os("XDG_DATA_HOME") {
        dirs_vec.push(PathBuf::from(dir));
    } else if let Some(home) = dirs::home_dir() {
        dirs_vec.push(home.join(".local/share"));
    }

    if let Ok(data_dirs) = std::env::var("XDG_DATA_DIRS") {
        for d in data_dirs.split(':') {
            dirs_vec.push(PathBuf::from(d));
        }
    } else {
        dirs_vec.push(PathBuf::from("/usr/local/share"));
        dirs_vec.push(PathBuf::from("/usr/share"));
    }
    dirs_vec
}

#[derive(Default)]
struct ThemeInfo {
    directories: Vec<String>,
    inherits: Vec<String>,
}

fn parse_index_theme(path: &Path) -> ThemeInfo {
    if let Ok(conf) = Ini::load_from_file(path) {
        let mut info = ThemeInfo::default();
        if let Some(section) = conf.section(Some("Icon Theme")) {
            if let Some(dirs) = section.get("Directories") {
                info.directories = dirs
                    .split(|c| c == ',' || c == ';')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
            if let Some(inherits) = section.get("Inherits") {
                info.inherits = inherits
                    .split(|c| c == ',' || c == ';')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
            }
        }
        info
    } else {
        ThemeInfo::default()
    }
}

fn resolve_icon(name: &str) -> Option<String> {
    let path = Path::new(name);
    if path.is_absolute() && path.exists() {
        return Some(path.to_string_lossy().into_owned());
    }

    let mut themes = Vec::new();
    if let Ok(theme) = std::env::var("XDG_ICON_THEME") {
        themes.push(theme);
    }
    if !themes.contains(&"hicolor".to_string()) {
        themes.push("hicolor".into());
    }

    let extensions = ["png", "svg", "xpm"];
    let base_dirs = icon_base_dirs();

    let mut visited = HashSet::new();
    while let Some(theme) = themes.pop() {
        if !visited.insert(theme.clone()) {
            continue;
        }
        for base in &base_dirs {
            let theme_dir = base.join("icons").join(&theme);
            let info = parse_index_theme(&theme_dir.join("index.theme"));

            for dir in &info.directories {
                for ext in &extensions {
                    let candidate = theme_dir.join(dir).join(format!("{}.{}", name, ext));
                    if candidate.exists() {
                        return Some(candidate.to_string_lossy().into_owned());
                    }
                }
            }

            for inherit in info.inherits {
                themes.push(inherit);
            }
        }
    }

    for base in base_dirs {
        for ext in &extensions {
            let candidate = base.join("pixmaps").join(format!("{}.{}", name, ext));
            if candidate.exists() {
                return Some(candidate.to_string_lossy().into_owned());
            }
        }
    }

    None
}

pub struct Dashboard {
    pub show_settings: bool,
    pub applications: Vec<AppInfo>,
}

impl Dashboard {
    fn find_applications() -> Vec<AppInfo> {
        let mut apps = Vec::new();
        let paths = vec![
            dirs::data_dir().unwrap_or_default().join("applications"),
            Path::new("/usr/share/applications").to_path_buf(),
        ];

        for dir in paths {
            if let Ok(entries) = fs::read_dir(dir) {
                for entry in entries.flatten() {
                    if entry.path().extension() == Some(OsStr::new("desktop")) {
                        if let Ok(content) = fs::read_to_string(entry.path()) {
                            let name = content.lines().find_map(|line| {
                                if line.starts_with("Name=") {
                                    Some(line.trim_start_matches("Name=").to_string())
                                } else {
                                    None
                                }
                            });
                            let exec = content.lines().find_map(|line| {
                                if line.starts_with("Exec=") {
                                    Some(
                                        line.trim_start_matches("Exec=")
                                            .split_whitespace()
                                            .next()
                                            .unwrap_or("")
                                            .to_string(),
                                    )
                                } else {
                                    None
                                }
                            });

                            let icon = content
                                .lines()
                                .find_map(|line| {
                                    if line.starts_with("Icon=") {
                                        Some(line.trim_start_matches("Icon=").to_string())
                                    } else {
                                        None
                                    }
                                })
                                .and_then(|n| resolve_icon(&n));

                            if let (Some(name), Some(exec)) = (name, exec) {
                                apps.push(AppInfo { name, exec, icon });
                            }
                        } else {
                            // ignore non-.desktop files, they don't describe applications
                        }
                    }
                }
            }
        }
        apps
    }
}

impl Default for Dashboard {
    fn default() -> Self {
        let apps = Self::find_applications();
        Self {
            show_settings: false,
            applications: apps,
        }
    }
}

impl Application for Dashboard {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Dashboard")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::LaunchApp(exec) => {
                let _ = std::process::Command::new(exec).spawn();
            }
            Message::ToggleSettings => self.show_settings = !self.show_settings,
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        if self.show_settings {
            settings_view()
        } else {
            launcher_view(&self.applications)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn parse_index_theme_handles_multiple_separators() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("index.theme");
        let mut file = File::create(&path).unwrap();
        writeln!(
            file,
            "[Icon Theme]\nDirectories=32x32/apps;48x48/apps,64x64/apps\nInherits=base;legacy,old"
        )
        .unwrap();

        let info = parse_index_theme(&path);
        assert_eq!(
            info.directories,
            vec!["32x32/apps", "48x48/apps", "64x64/apps"]
        );
        assert_eq!(info.inherits, vec!["base", "legacy", "old"]);
    }
}

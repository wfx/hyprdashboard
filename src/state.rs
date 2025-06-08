use crate::message::Message;
use crate::ui::{launcher_view, settings_view};
use iced::{Application, Command, Element, Theme};

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

#[derive(Debug)]
pub struct AppInfo {
    pub name: String,
    pub exec: String,
    pub icon: Option<String>,
}

fn resolve_icon(name: &str) -> Option<String> {
    let path = Path::new(name);
    if path.is_absolute() && path.exists() {
        return Some(path.to_string_lossy().into_owned());
    }

    let search_dirs = vec![
        dirs::data_dir().unwrap_or_default().join("icons"),
        PathBuf::from("/usr/share/icons"),
        PathBuf::from("/usr/share/pixmaps"),
    ];

    let extensions = ["png", "svg", "xpm"];

    for dir in search_dirs {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                if let Some(stem) = entry.path().file_stem() {
                    if stem == name {
                        if let Some(ext) = entry.path().extension() {
                            if extensions.contains(&ext.to_string_lossy().as_ref()) {
                                return Some(entry.path().to_string_lossy().into_owned());
                            }
                        }
                    }
                }
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

use crate::message::Message;
use crate::ui::{launcher_view, settings_view};
use iced::{Application, Command, Element, Theme};

use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct AppInfo {
    pub name: String,
    pub exec: String,
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

                        if let (Some(name), Some(exec)) = (name, exec) {
                            apps.push(AppInfo { name, exec });
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
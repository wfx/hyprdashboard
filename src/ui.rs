use crate::message::Message;
use crate::state::AppInfo;
use iced::alignment::Horizontal;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Length};

pub fn launcher_view(apps: &[AppInfo]) -> Element<'static, Message> {
    let mut content = column![
        text("DATE:DDDD YYYY:MM:DD and TIME:HH:MM")
            .size(14)
            .horizontal_alignment(Horizontal::Center),
        row![
            button("Network").on_press(Message::ToggleSettings),
            button("Bluetooth"),
            button("Audio Volume"),
            button("Settings").on_press(Message::ToggleSettings),
        ]
        .spacing(10)
        .padding(10)
        .align_items(iced::Alignment::Center),
    ]
    .spacing(10)
    .padding(20);

    for app in apps {
        content = content.push(
            button(text(&app.name))
                .on_press(Message::LaunchApp(app.exec.clone()))
                .width(Length::Fill)
                .padding(10),
        );
    }

    container(scrollable(content).height(Length::Fill))
        .padding(20)
        .into()
}

pub fn settings_view() -> Element<'static, Message> {
    let content = column![
        text("Settings Menu").size(24),
        row![
            column![button("Network"), button("Audio"), button("Hyprland")].spacing(10),
            column![
                text("Network Settings ..."),
                text("Audio Settings ..."),
                text("Hyprland Settings ..."),
            ]
            .spacing(10),
        ]
        .spacing(20),
        button("Back").on_press(Message::ToggleSettings),
    ]
    .spacing(20)
    .padding(20);

    container(scrollable(content).height(Length::Fill))
        .padding(20)
        .into()
}

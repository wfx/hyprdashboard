use crate::message::Message;
use crate::state::AppInfo;
use iced::alignment::Horizontal;
use iced::widget::{button, column, container, image, row, scrollable, text};
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

    const COLS: usize = 4;
    let mut row_view = row![];
    let mut count = 0;
    for app in apps {
        let mut btn_content = column![];
        if let Some(icon) = &app.icon {
            btn_content = btn_content.push(
                image(icon.clone())
                    .width(64)
                    .height(64),
            );
        }
        btn_content = btn_content.push(text(&app.name));
        row_view = row_view.push(
            button(btn_content)
                .on_press(Message::LaunchApp(app.exec.clone()))
                .width(Length::Fill)
                .padding(5),
        );
        count += 1;
        if count == COLS {
            content = content.push(row_view);
            row_view = row![];
            count = 0;
        }
    }
    if count > 0 {
        content = content.push(row_view);
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

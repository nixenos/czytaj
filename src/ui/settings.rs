use iced::widget::{button, column, container, pick_list, text, Column};
use iced::{color, Element, Length, Padding, Theme};
use crate::models::{AppSettings, AppTheme};

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    ThemeSelected(AppTheme),
    ToggleImages,
    ToggleExcerpts,
    CloseSettings,
}

pub fn settings_view<'a>(settings: &'a AppSettings) -> Element<'a, SettingsMessage> {
    let mut content = Column::new()
        .spacing(20)
        .padding(Padding::from([30, 40]));

    content = content.push(
        text("Settings")
            .size(32)
            .color(color!(0xE0E0E0))
    );

    content = content.push(
        container(column![])
            .height(2)
            .width(Length::Fill)
            .style(|_theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(color!(0x3A3A3A))),
                    ..Default::default()
                }
            })
    );

    // Theme selector
    content = content.push(
        column![
            text("Theme")
                .size(18)
                .color(color!(0xE0E0E0)),
            pick_list(
                AppTheme::all(),
                Some(settings.theme),
                SettingsMessage::ThemeSelected,
            )
            .placeholder("Select a theme...")
            .text_size(14)
            .padding(10)
            .width(Length::Fixed(300.0))
            .style(|_theme: &Theme, _status| {
                pick_list::Style {
                    text_color: color!(0xE0E0E0),
                    background: iced::Background::Color(color!(0x2A2A2A)),
                    placeholder_color: color!(0x606060),
                    handle_color: color!(0x808080),
                    border: iced::Border {
                        color: color!(0x3A3A3A),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                }
            })
        ]
        .spacing(10)
    );

    // Show images toggle
    let images_btn_text = if settings.show_images {
        "✓ Show Images"
    } else {
        "✗ Show Images"
    };
    content = content.push(
        button(text(images_btn_text).size(14).color(color!(0xE0E0E0)))
            .on_press(SettingsMessage::ToggleImages)
            .padding([10, 20])
            .style(move |_theme: &Theme, _status| {
                button::Style {
                    background: Some(iced::Background::Color(
                        if settings.show_images {
                            color!(0x3A7CA5)
                        } else {
                            color!(0x2A2A2A)
                        }
                    )),
                    text_color: color!(0xE0E0E0),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
    );

    // Show excerpts toggle
    let excerpts_btn_text = if settings.show_excerpts {
        "✓ Show Excerpts"
    } else {
        "✗ Show Excerpts"
    };
    content = content.push(
        button(text(excerpts_btn_text).size(14).color(color!(0xE0E0E0)))
            .on_press(SettingsMessage::ToggleExcerpts)
            .padding([10, 20])
            .style(move |_theme: &Theme, _status| {
                button::Style {
                    background: Some(iced::Background::Color(
                        if settings.show_excerpts {
                            color!(0x3A7CA5)
                        } else {
                            color!(0x2A2A2A)
                        }
                    )),
                    text_color: color!(0xE0E0E0),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
    );

    content = content.push(
        container(column![])
            .height(2)
            .width(Length::Fill)
            .style(|_theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(color!(0x3A3A3A))),
                    ..Default::default()
                }
            })
    );

    // Close button
    content = content.push(
        button(text("Close").size(14).color(color!(0xE0E0E0)))
            .on_press(SettingsMessage::CloseSettings)
            .padding([10, 20])
            .style(|_theme: &Theme, _status| {
                button::Style {
                    background: Some(iced::Background::Color(color!(0x3A7CA5))),
                    text_color: color!(0xE0E0E0),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
    );

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center(Length::Fill)
        .style(|_theme: &Theme| {
            container::Style {
                background: Some(iced::Background::Color(color!(0x1A1A1A))),
                ..Default::default()
            }
        })
        .into()
}

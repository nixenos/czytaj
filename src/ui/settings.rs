use iced::widget::{button, column, container, pick_list, text, Column};
use iced::{Element, Length, Padding, Shadow, Theme};
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
        .spacing(24)
        .padding(Padding::from([40, 50]))
        .max_width(600);

    // Modern header
    content = content.push(
        text("Settings")
            .size(40)
            .style(|theme: &Theme| {
                text::Style {
                    color: Some(theme.palette().text),
                }
            })
    );

    // Material divider
    content = content.push(
        container(column![])
            .height(2)
            .width(Length::Fill)
            .style(|theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(theme.extended_palette().primary.strong.color)),
                    border: iced::Border {
                        radius: 1.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
    );

    // Theme selector section with Material card
    content = content.push(
        container(
            column![
                text("Appearance")
                    .size(20)
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.palette().text),
                        }
                    }),
                text("Choose your preferred theme")
                    .size(14)
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.extended_palette().background.strong.text),
                        }
                    }),
                pick_list(
                    AppTheme::all(),
                    Some(settings.theme),
                    SettingsMessage::ThemeSelected,
                )
                .placeholder("Select a theme...")
                .text_size(15)
                .padding(12)
                .width(Length::Fill)
                .style(|theme: &Theme, status| {
                    let palette = theme.extended_palette();
                    pick_list::Style {
                        text_color: palette.background.base.text,
                        background: iced::Background::Color(palette.background.weak.color),
                        placeholder_color: palette.background.strong.text,
                        handle_color: palette.secondary.base.color,
                        border: iced::Border {
                            color: if matches!(status, pick_list::Status::Active) {
                                palette.primary.strong.color
                            } else {
                                palette.background.strong.color
                            },
                            width: if matches!(status, pick_list::Status::Active) { 2.0 } else { 1.0 },
                            radius: 8.0.into(),
                        },
                    }
                })
            ]
            .spacing(12)
            .padding(Padding::from([20, 24]))
        )
        .width(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(iced::Background::Color(palette.background.weak.color)),
                border: iced::Border {
                    color: palette.background.strong.color,
                    width: 0.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                ..Default::default()
            }
        })
    );

    // Content options section
    content = content.push(
        container(
            column![
                text("Content Display")
                    .size(20)
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.palette().text),
                        }
                    }),
                text("Customize what appears in articles")
                    .size(14)
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.extended_palette().background.strong.text),
                        }
                    }),
                // Toggle buttons container
                column![
                    create_toggle_button(
                        if settings.show_images { "✓ Show Images" } else { "Show Images" },
                        settings.show_images,
                        SettingsMessage::ToggleImages
                    ),
                    create_toggle_button(
                        if settings.show_excerpts { "✓ Show Excerpts" } else { "Show Excerpts" },
                        settings.show_excerpts,
                        SettingsMessage::ToggleExcerpts
                    ),
                ]
                .spacing(12)
            ]
            .spacing(12)
            .padding(Padding::from([20, 24]))
        )
        .width(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(iced::Background::Color(palette.background.weak.color)),
                border: iced::Border {
                    color: palette.background.strong.color,
                    width: 0.0,
                    radius: 12.0.into(),
                },
                shadow: Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
                ..Default::default()
            }
        })
    );

    content = content.push(
        container(column![])
            .height(1)
            .width(Length::Fill)
            .style(|theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(theme.extended_palette().background.strong.color)),
                    ..Default::default()
                }
            })
    );

    // Close button with Material FAB style
    content = content.push(
        button(text("Close Settings").size(16))
            .on_press(SettingsMessage::CloseSettings)
            .padding([14, 32])
            .width(Length::Fill)
            .style(|theme: &Theme, status| {
                let palette = theme.extended_palette();
                let base = button::Style {
                    background: Some(iced::Background::Color(palette.primary.strong.color)),
                    text_color: palette.primary.strong.text,
                    border: iced::Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    shadow: Shadow {
                        color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 4.0,
                    },
                };
                
                match status {
                    button::Status::Hovered => button::Style {
                        shadow: Shadow {
                            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.25),
                            offset: iced::Vector::new(0.0, 4.0),
                            blur_radius: 8.0,
                        },
                        ..base
                    },
                    button::Status::Pressed => button::Style {
                        background: Some(iced::Background::Color(palette.primary.base.color)),
                        shadow: Shadow {
                            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                            offset: iced::Vector::new(0.0, 1.0),
                            blur_radius: 2.0,
                        },
                        ..base
                    },
                    _ => base,
                }
            })
    );

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center(Length::Fill)
        .style(|theme: &Theme| {
            container::Style {
                background: Some(iced::Background::Color(theme.extended_palette().background.base.color)),
                ..Default::default()
            }
        })
        .into()
}

fn create_toggle_button(label: &str, is_active: bool, message: SettingsMessage) -> Element<'_, SettingsMessage> {
    button(text(label).size(15))
        .on_press(message)
        .padding([12, 20])
        .width(Length::Fill)
        .style(move |theme: &Theme, status| {
            let palette = theme.extended_palette();
            let base = button::Style {
                background: Some(iced::Background::Color(
                    if is_active {
                        palette.primary.strong.color
                    } else {
                        palette.background.strong.color
                    }
                )),
                text_color: if is_active {
                    palette.primary.strong.text
                } else {
                    palette.background.base.text
                },
                border: iced::Border {
                    radius: 8.0.into(),
                    ..Default::default()
                },
                shadow: Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                    offset: iced::Vector::new(0.0, 1.0),
                    blur_radius: 3.0,
                },
            };
            
            match status {
                button::Status::Hovered => button::Style {
                    background: Some(iced::Background::Color(
                        if is_active {
                            palette.primary.base.color
                        } else {
                            palette.secondary.weak.color
                        }
                    )),
                    shadow: Shadow {
                        color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 6.0,
                    },
                    ..base
                },
                button::Status::Pressed => button::Style {
                    shadow: Shadow {
                        color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.05),
                        offset: iced::Vector::new(0.0, 1.0),
                        blur_radius: 2.0,
                    },
                    ..base
                },
                _ => base,
            }
        })
        .into()
}

use iced::widget::{button, column, container, text, text_input, Column};
use iced::{Element, Length, Padding, Shadow, Theme};
use crate::models::Feed;

#[derive(Debug, Clone)]
pub enum SidebarMessage {
    FeedInputChanged(String),
    AddFeed,
    RefreshFeed(String),
    OpenSettings,
}

pub fn sidebar_view<'a>(
    feeds: &'a [Feed],
    feed_input: &'a str,
) -> Element<'a, SidebarMessage> {
    let mut feed_list = Column::new()
        .spacing(12)
        .padding(Padding::from([16, 20]));

    // Modern header with theme-aware colors
    feed_list = feed_list.push(
        text("Feeds")
            .size(28)
            .style(|theme: &Theme| {
                text::Style {
                    color: Some(theme.palette().text),
                }
            })
    );

    // Material-style divider with theme colors
    feed_list = feed_list.push(
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

    // Feed list with Material card style and hover effects
    for feed in feeds {
        feed_list = feed_list.push(
            button(
                text(&feed.title)
                    .size(15)
            )
            .on_press(SidebarMessage::RefreshFeed(feed.url.clone()))
            .padding([12, 16])
            .width(Length::Fill)
            .style(|theme: &Theme, status| {
                let palette = theme.extended_palette();
                let base = button::Style {
                    background: Some(iced::Background::Color(palette.background.weak.color)),
                    text_color: palette.background.weak.text,
                    border: iced::Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    shadow: Shadow {
                        color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: iced::Vector::new(0.0, 2.0),
                        blur_radius: 4.0,
                    },
                };
                
                match status {
                    button::Status::Hovered => button::Style {
                        background: Some(iced::Background::Color(palette.background.strong.color)),
                        shadow: Shadow {
                            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                            offset: iced::Vector::new(0.0, 4.0),
                            blur_radius: 8.0,
                        },
                        ..base
                    },
                    button::Status::Pressed => button::Style {
                        background: Some(iced::Background::Color(palette.primary.weak.color)),
                        ..base
                    },
                    _ => base,
                }
            })
        );
    }

    feed_list = feed_list.push(
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

    // Section header
    feed_list = feed_list.push(
        text("Add Feed")
            .size(18)
            .style(|theme: &Theme| {
                text::Style {
                    color: Some(theme.palette().text),
                }
            })
    );

    // Modern input field with theme colors
    feed_list = feed_list.push(
        text_input("Enter feed URL...", feed_input)
            .on_input(SidebarMessage::FeedInputChanged)
            .on_submit(SidebarMessage::AddFeed)
            .padding(12)
            .size(15)
            .style(|theme: &Theme, status| {
                let palette = theme.extended_palette();
                text_input::Style {
                    background: iced::Background::Color(palette.background.weak.color),
                    border: iced::Border {
                        color: if matches!(status, text_input::Status::Focused) {
                            palette.primary.strong.color
                        } else {
                            palette.background.strong.color
                        },
                        width: if matches!(status, text_input::Status::Focused) { 2.0 } else { 1.0 },
                        radius: 8.0.into(),
                    },
                    icon: palette.background.base.text,
                    placeholder: palette.background.strong.text,
                    value: palette.background.base.text,
                    selection: palette.primary.weak.color,
                }
            })
    );

    // Material FAB-style button
    feed_list = feed_list.push(
        button(text("Add Feed").size(15))
            .on_press(SidebarMessage::AddFeed)
            .padding([12, 24])
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

    feed_list = feed_list.push(
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

    // Settings button with icon
    feed_list = feed_list.push(
        button(text("⚙ Settings").size(15))
            .on_press(SidebarMessage::OpenSettings)
            .padding([12, 24])
            .width(Length::Fill)
            .style(|theme: &Theme, status| {
                let palette = theme.extended_palette();
                let base = button::Style {
                    background: Some(iced::Background::Color(palette.background.weak.color)),
                    text_color: palette.background.base.text,
                    border: iced::Border {
                        radius: 8.0.into(),
                        ..Default::default()
                    },
                    shadow: Shadow {
                        color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.1),
                        offset: iced::Vector::new(0.0, 1.0),
                        blur_radius: 2.0,
                    },
                };
                
                match status {
                    button::Status::Hovered => button::Style {
                        background: Some(iced::Background::Color(palette.background.strong.color)),
                        shadow: Shadow {
                            color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                            offset: iced::Vector::new(0.0, 3.0),
                            blur_radius: 6.0,
                        },
                        ..base
                    },
                    button::Status::Pressed => button::Style {
                        background: Some(iced::Background::Color(palette.secondary.weak.color)),
                        ..base
                    },
                    _ => base,
                }
            })
    );

    // Sidebar container with Material elevation
    container(feed_list)
        .width(300)
        .height(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(iced::Background::Color(palette.background.base.color)),
                border: iced::Border {
                    color: palette.background.strong.color,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                    offset: iced::Vector::new(2.0, 0.0),
                    blur_radius: 8.0,
                },
                ..Default::default()
            }
        })
        .into()
}

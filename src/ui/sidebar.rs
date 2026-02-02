use iced::widget::{button, column, container, text, text_input, Column};
use iced::{color, Element, Length, Padding, Theme};
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
        .spacing(8)
        .padding(Padding::from([12, 16]));

    feed_list = feed_list.push(
        text("Feeds")
            .size(24)
            .color(color!(0xE0E0E0))
    );

    feed_list = feed_list.push(
        container(column![])
            .height(1)
            .width(Length::Fill)
            .style(|_theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(color!(0x3A3A3A))),
                    ..Default::default()
                }
            })
    );

    for feed in feeds {
        feed_list = feed_list.push(
            button(
                text(&feed.title)
                    .size(14)
                    .color(color!(0xB0B0B0))
            )
            .on_press(SidebarMessage::RefreshFeed(feed.url.clone()))
            .padding([8, 12])
            .style(|_theme: &Theme, _status| {
                button::Style {
                    background: Some(iced::Background::Color(color!(0x2A2A2A))),
                    text_color: color!(0xB0B0B0),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
        );
    }

    feed_list = feed_list.push(
        container(column![])
            .height(1)
            .width(Length::Fill)
            .style(|_theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(color!(0x3A3A3A))),
                    ..Default::default()
                }
            })
    );

    feed_list = feed_list.push(
        text("Add Feed")
            .size(16)
            .color(color!(0xE0E0E0))
    );

    feed_list = feed_list.push(
        text_input("Enter feed URL...", feed_input)
            .on_input(SidebarMessage::FeedInputChanged)
            .on_submit(SidebarMessage::AddFeed)
            .padding(10)
            .size(14)
            .style(|_theme: &Theme, _status| {
                text_input::Style {
                    background: iced::Background::Color(color!(0x2A2A2A)),
                    border: iced::Border {
                        color: color!(0x3A3A3A),
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    icon: color!(0x808080),
                    placeholder: color!(0x606060),
                    value: color!(0xE0E0E0),
                    selection: color!(0x4A4A4A),
                }
            })
    );

    feed_list = feed_list.push(
        button(text("Add").size(14).color(color!(0xE0E0E0)))
            .on_press(SidebarMessage::AddFeed)
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

    feed_list = feed_list.push(
        container(column![])
            .height(1)
            .width(Length::Fill)
            .style(|_theme: &Theme| {
                container::Style {
                    background: Some(iced::Background::Color(color!(0x3A3A3A))),
                    ..Default::default()
                }
            })
    );

    feed_list = feed_list.push(
        button(text("⚙ Settings").size(14).color(color!(0xE0E0E0)))
            .on_press(SidebarMessage::OpenSettings)
            .padding([10, 20])
            .style(|_theme: &Theme, _status| {
                button::Style {
                    background: Some(iced::Background::Color(color!(0x2A2A2A))),
                    text_color: color!(0xE0E0E0),
                    border: iced::Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
    );

    container(feed_list)
        .width(280)
        .height(Length::Fill)
        .style(|_theme: &Theme| {
            container::Style {
                background: Some(iced::Background::Color(color!(0x1E1E1E))),
                border: iced::Border {
                    color: color!(0x3A3A3A),
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
}

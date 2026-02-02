use iced::widget::{button, column, container, scrollable, text, Column};
use iced::{Element, Length, Padding, Shadow, Theme};
use crate::models::Article;

#[derive(Debug, Clone)]
pub enum ArticleDetailMessage {
    BackToList,
}

pub fn article_detail_view<'a>(article: &'a Article) -> Element<'a, ArticleDetailMessage> {
    let mut content = Column::new()
        .spacing(24)
        .padding(Padding::from([32, 40]))
        .max_width(900);

    // Back button with Material Design styling
    let back_button = button(
        text("← Back to Articles")
            .size(16)
    )
    .on_press(ArticleDetailMessage::BackToList)
    .padding(Padding::from([12, 24]))
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
                background: Some(iced::Background::Color(palette.secondary.weak.color)),
                shadow: Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                    offset: iced::Vector::new(0.0, 1.0),
                    blur_radius: 2.0,
                },
                ..base
            },
            _ => base,
        }
    });
    
    content = content.push(back_button);

    // Article title with theme colors
    content = content.push(
        text(&article.title)
            .size(36)
            .style(|theme: &Theme| {
                text::Style {
                    color: Some(theme.palette().text),
                }
            })
    );

    // Material-style divider
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

    // Image if available
    if let Some(image_url) = &article.image_url {
        content = content.push(
            container(
                text(format!("🖼 Image: {}", image_url))
                    .size(14)
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.extended_palette().secondary.base.color),
                        }
                    })
            )
            .padding(Padding::from([10, 0]))
        );
    }

    // Article excerpt/content with better styling
    if let Some(excerpt) = &article.excerpt {
        content = content.push(
            text(excerpt)
                .size(18)
                .style(|theme: &Theme| {
                    text::Style {
                        color: Some(theme.palette().text),
                    }
                })
                .line_height(1.6)
        );
    } else {
        content = content.push(
            text("No content preview available.")
                .size(16)
                .style(|theme: &Theme| {
                    text::Style {
                        color: Some(theme.extended_palette().background.strong.text),
                    }
                })
        );
    }

    // Article link in a card
    content = content.push(
        container(
            column![
                text("Source URL:")
                    .size(14)
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.extended_palette().background.strong.text),
                        }
                    }),
                text(&article.link)
                    .size(14)
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.extended_palette().primary.base.color),
                        }
                    })
            ]
            .spacing(6)
            .padding(Padding::from([16, 20]))
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
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.08),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 6.0,
                },
                ..Default::default()
            }
        })
    );

    let scrollable_content = scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill);

    container(scrollable_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .style(|theme: &Theme| {
            container::Style {
                background: Some(iced::Background::Color(theme.extended_palette().background.base.color)),
                ..Default::default()
            }
        })
        .into()
}

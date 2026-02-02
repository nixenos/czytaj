use iced::widget::{button, column, container, scrollable, text, Column};
use iced::{Element, Length, Padding, Shadow, Theme};
use crate::models::{Article, AppSettings};
use crate::db::ArticleDatabase;

#[derive(Debug, Clone)]
pub enum ContentMessage {
    ArticleClicked(Article),
}

pub fn content_view<'a>(
    articles: &'a [Article],
    loading: bool,
    settings: &'a AppSettings,
    db: &'a ArticleDatabase,
) -> Element<'a, ContentMessage> {
    let mut article_list = Column::new()
        .spacing(20)
        .padding(Padding::from([24, 32]));

    // Modern header with theme-aware colors
    article_list = article_list.push(
        text("Articles")
            .size(34)
            .style(|theme: &Theme| {
                text::Style {
                    color: Some(theme.palette().text),
                }
            })
    );

    // Material-style divider
    article_list = article_list.push(
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

    if loading {
        // Animated loading state
        article_list = article_list.push(
            container(
                text("⟳ Loading articles...")
                    .size(18)
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.extended_palette().secondary.base.color),
                        }
                    })
            )
            .padding(40)
            .center(Length::Fill)
        );
    } else if articles.is_empty() {
        // Empty state with helpful message
        article_list = article_list.push(
            container(
                column![
                    text("📰")
                        .size(64),
                    text("No articles yet")
                        .size(24)
                        .style(|theme: &Theme| {
                            text::Style {
                                color: Some(theme.palette().text),
                            }
                        }),
                    text("Add a feed to get started!")
                        .size(16)
                        .style(|theme: &Theme| {
                            text::Style {
                                color: Some(theme.extended_palette().background.strong.text),
                            }
                        }),
                ]
                .spacing(12)
                .align_x(iced::Alignment::Center)
            )
            .padding(60)
            .center(Length::Fill)
            .width(Length::Fill)
        );
    } else {
        // Article cards with Material Design elevation
        for article in articles {
            let is_viewed = db.is_viewed(&article.link).unwrap_or(false);
            
            let mut article_content = Column::new().spacing(10);

            // Article title with prominent styling
            article_content = article_content.push(
                text(&article.title)
                    .size(20)
                    .style(move |theme: &Theme| {
                        text::Style {
                            color: Some(if is_viewed {
                                theme.extended_palette().background.strong.text
                            } else {
                                theme.palette().text
                            }),
                        }
                    })
            );

            // Image indicator if available and enabled
            if settings.show_images {
                if let Some(image_url) = &article.image_url {
                    article_content = article_content.push(
                        text(format!("🖼 {}", truncate_text(image_url, 60)))
                            .size(12)
                            .style(|theme: &Theme| {
                                text::Style {
                                    color: Some(theme.extended_palette().secondary.base.color),
                                }
                            })
                    );
                }
            }

            // Excerpt with proper styling
            if settings.show_excerpts {
                if let Some(excerpt) = &article.excerpt {
                    article_content = article_content.push(
                        text(excerpt)
                            .size(14)
                            .style(|theme: &Theme| {
                                text::Style {
                                    color: Some(theme.extended_palette().background.base.text),
                                }
                            })
                    );
                }
            }

            // Link with accent color
            article_content = article_content.push(
                text(&article.link)
                    .size(13)
                    .style(|theme: &Theme| {
                        text::Style {
                            color: Some(theme.extended_palette().primary.base.color),
                        }
                    })
            );

            // Material card with elevation and theme colors wrapped in button
            let article_card = container(article_content.padding(Padding::from([20, 24])))
                .width(Length::Fill)
                .style(move |theme: &Theme| {
                    let palette = theme.extended_palette();
                    container::Style {
                        background: Some(iced::Background::Color(
                            if is_viewed {
                                palette.background.base.color
                            } else {
                                palette.background.weak.color
                            }
                        )),
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
                });

            let article_button = button(article_card)
                .on_press(ContentMessage::ArticleClicked(article.clone()))
                .padding(0)
                .style(|theme: &Theme, status| {
                    let base = button::Style {
                        background: None,
                        text_color: theme.palette().text,
                        border: iced::Border::default(),
                        shadow: Shadow::default(),
                    };
                    
                    match status {
                        button::Status::Hovered => button::Style {
                            shadow: Shadow {
                                color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.15),
                                offset: iced::Vector::new(0.0, 4.0),
                                blur_radius: 12.0,
                            },
                            ..base
                        },
                        _ => base,
                    }
                });

            article_list = article_list.push(article_button);
        }
    }

    let scrollable_content = scrollable(article_list)
        .width(Length::Fill)
        .height(Length::Fill);

    container(scrollable_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| {
            container::Style {
                background: Some(iced::Background::Color(theme.extended_palette().background.base.color)),
                ..Default::default()
            }
        })
        .into()
}

fn truncate_text(text: &str, max_len: usize) -> String {
    if text.len() <= max_len {
        text.to_string()
    } else {
        format!("{}...", &text[..max_len])
    }
}

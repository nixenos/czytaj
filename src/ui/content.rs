use iced::widget::{button, column, container, scrollable, text, Column};
use iced::{color, Element, Length, Padding, Theme};
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
        .spacing(16)
        .padding(Padding::from([20, 24]));

    article_list = article_list.push(
        text("Articles")
            .size(28)
            .color(color!(0xE0E0E0))
    );

    article_list = article_list.push(
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

    if loading {
        article_list = article_list.push(
            text("Loading...")
                .size(16)
                .color(color!(0x808080))
        );
    } else if articles.is_empty() {
        article_list = article_list.push(
            text("No articles yet. Add a feed to get started!")
                .size(16)
                .color(color!(0x808080))
        );
    } else {
        for article in articles {
            let mut article_content = Column::new().spacing(6);

            // Check if article was viewed
            let is_viewed = db.is_viewed(&article.link).unwrap_or(false);
            let title_color = if is_viewed {
                color!(0x808080) // Gray for viewed articles
            } else {
                color!(0xE0E0E0) // Bright for unviewed articles
            };

            // Add article title with viewed indicator
            let title_text = if is_viewed {
                format!("✓ {}", &article.title)
            } else {
                article.title.clone()
            };
            
            article_content = article_content.push(
                text(title_text)
                    .size(18)
                    .color(title_color)
            );

            // Add image if available and enabled in settings
            if settings.show_images {
                if let Some(image_url) = &article.image_url {
                    // For now, just show the URL since iced::widget::Image needs local paths
                    // In a real implementation, you'd download and cache images
                    article_content = article_content.push(
                        text(format!("🖼️ Image: {}", truncate_text(image_url, 50)))
                            .size(11)
                            .color(color!(0x606060))
                    );
                }
            }

            // Add excerpt if available and enabled in settings
            if settings.show_excerpts {
                if let Some(excerpt) = &article.excerpt {
                    article_content = article_content.push(
                        text(excerpt)
                            .size(13)
                            .color(color!(0x909090))
                    );
                }
            }

            // Add link
            article_content = article_content.push(
                text(&article.link)
                    .size(12)
                    .color(color!(0x808080))
            );

            // Make the entire article clickable
            let article_button = button(
                container(article_content.padding(Padding::from([16, 20])))
                    .width(Length::Fill)
            )
            .on_press(ContentMessage::ArticleClicked(article.clone()))
            .padding(0)
            .style(move |_theme: &Theme, status| {
                let background_color = match status {
                    button::Status::Hovered => color!(0x2A2A2A),
                    button::Status::Pressed => color!(0x202020),
                    _ => color!(0x252525),
                };
                
                button::Style {
                    background: Some(iced::Background::Color(background_color)),
                    border: iced::Border {
                        color: color!(0x3A3A3A),
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    text_color: color!(0xE0E0E0),
                    ..Default::default()
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
        .style(|_theme: &Theme| {
            container::Style {
                background: Some(iced::Background::Color(color!(0x1A1A1A))),
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

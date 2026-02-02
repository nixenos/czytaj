use iced::widget::{button, column, container, scrollable, text, Column};
use iced::{color, Element, Length, Padding, Theme};
use crate::models::Article;

#[derive(Debug, Clone)]
pub enum ArticleDetailMessage {
    BackToList,
}

pub fn article_detail_view<'a>(article: &'a Article) -> Element<'a, ArticleDetailMessage> {
    let mut content = Column::new()
        .spacing(20)
        .padding(Padding::from([20, 24]))
        .max_width(900);

    // Back button
    let back_button = button(
        text("← Back to Articles")
            .size(16)
    )
    .on_press(ArticleDetailMessage::BackToList)
    .padding(Padding::from([10, 20]))
    .style(|_theme: &Theme, status| {
        let background_color = match status {
            button::Status::Hovered => color!(0x3A3A3A),
            button::Status::Pressed => color!(0x2A2A2A),
            _ => color!(0x303030),
        };
        
        button::Style {
            background: Some(iced::Background::Color(background_color)),
            border: iced::Border {
                color: color!(0x4A4A4A),
                width: 1.0,
                radius: 6.0.into(),
            },
            text_color: color!(0xE0E0E0),
            ..Default::default()
        }
    });
    
    content = content.push(back_button);

    // Article title
    content = content.push(
        text(&article.title)
            .size(32)
            .color(color!(0xE0E0E0))
    );

    // Divider
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

    // Image if available
    if let Some(image_url) = &article.image_url {
        content = content.push(
            container(
                text(format!("🖼️ Image: {}", image_url))
                    .size(14)
                    .color(color!(0x909090))
            )
            .padding(Padding::from([10, 0]))
        );
    }

    // Article excerpt/content
    if let Some(excerpt) = &article.excerpt {
        content = content.push(
            text(excerpt)
                .size(16)
                .color(color!(0xC0C0C0))
                .line_height(1.6)
        );
    } else {
        content = content.push(
            text("No content preview available.")
                .size(16)
                .color(color!(0x808080))
        );
    }

    // Article link
    content = content.push(
        container(
            column![
                text("Source URL:")
                    .size(14)
                    .color(color!(0x808080)),
                text(&article.link)
                    .size(14)
                    .color(color!(0x6A9FB5))
            ]
            .spacing(4)
        )
        .padding(Padding::new(20.0).top(20.0))
    );

    let scrollable_content = scrollable(content)
        .width(Length::Fill)
        .height(Length::Fill);

    container(scrollable_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .style(|_theme: &Theme| {
            container::Style {
                background: Some(iced::Background::Color(color!(0x1A1A1A))),
                ..Default::default()
            }
        })
        .into()
}

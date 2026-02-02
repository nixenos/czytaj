use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};
use iced::{color, Element, Length, Padding, Task, Theme};

mod feed_engine;
use feed_engine::{Article, FeedEngine};

fn main() -> iced::Result {
    iced::application("Czytaj - RSS Reader", App::update, App::view)
        .theme(App::theme)
        .run_with(App::new)
}

#[derive(Debug, Clone)]
struct Feed {
    url: String,
    title: String,
}

struct App {
    feeds: Vec<Feed>,
    articles: Vec<Article>,
    feed_input: String,
    loading: bool,
}

#[derive(Debug, Clone)]
enum Message {
    FeedInputChanged(String),
    AddFeed,
    FeedsFetched(Result<Vec<Article>, String>),
    RefreshFeed(String),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                feeds: vec![],
                articles: vec![],
                feed_input: String::new(),
                loading: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FeedInputChanged(value) => {
                self.feed_input = value;
                Task::none()
            }
            Message::AddFeed => {
                if self.feed_input.trim().is_empty() {
                    return Task::none();
                }

                let url = self.feed_input.clone();
                self.feeds.push(Feed {
                    url: url.clone(),
                    title: url.clone(),
                });
                self.feed_input.clear();
                self.loading = true;

                Task::perform(
                    FeedEngine::fetch_feed(url),
                    Message::FeedsFetched,
                )
            }
            Message::RefreshFeed(url) => {
                self.loading = true;
                Task::perform(
                    FeedEngine::fetch_feed(url),
                    Message::FeedsFetched,
                )
            }
            Message::FeedsFetched(result) => {
                self.loading = false;
                match result {
                    Ok(articles) => {
                        self.articles = articles;
                    }
                    Err(e) => {
                        eprintln!("Error fetching feed: {}", e);
                    }
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content = row![
            self.sidebar(),
            self.content(),
        ]
        .spacing(0);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
    }

    fn sidebar(&self) -> Element<'_, Message> {
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

        for feed in &self.feeds {
            feed_list = feed_list.push(
                button(
                    text(&feed.title)
                        .size(14)
                        .color(color!(0xB0B0B0))
                )
                .on_press(Message::RefreshFeed(feed.url.clone()))
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
            text_input("Enter feed URL...", &self.feed_input)
                .on_input(Message::FeedInputChanged)
                .on_submit(Message::AddFeed)
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
                .on_press(Message::AddFeed)
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

    fn content(&self) -> Element<'_, Message> {
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

        if self.loading {
            article_list = article_list.push(
                text("Loading...")
                    .size(16)
                    .color(color!(0x808080))
            );
        } else if self.articles.is_empty() {
            article_list = article_list.push(
                text("No articles yet. Add a feed to get started!")
                    .size(16)
                    .color(color!(0x808080))
            );
        } else {
            for article in &self.articles {
                let article_item = column![
                    text(&article.title)
                        .size(18)
                        .color(color!(0xE0E0E0)),
                    text(&article.link)
                        .size(12)
                        .color(color!(0x808080)),
                ]
                .spacing(6)
                .padding(Padding::from([16, 20]));

                article_list = article_list.push(
                    container(article_item)
                        .width(Length::Fill)
                        .style(|_theme: &Theme| {
                            container::Style {
                                background: Some(iced::Background::Color(color!(0x252525))),
                                border: iced::Border {
                                    color: color!(0x3A3A3A),
                                    width: 1.0,
                                    radius: 6.0.into(),
                                },
                                ..Default::default()
                            }
                        })
                );
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

    fn theme(&self) -> Theme {
        Theme::TokyoNightStorm
    }
}

use iced::{Element, Task, Theme};

mod feed_engine;
mod models;
mod ui;
mod utils;

use models::{AppSettings, Article, Feed, FeedData};
use ui::{content::ContentMessage, settings::SettingsMessage, sidebar::SidebarMessage};

fn main() -> iced::Result {
    iced::application("Czytaj - RSS Reader", App::update, App::view)
        .theme(App::theme)
        .run_with(App::new)
}

struct App {
    feeds: Vec<Feed>,
    articles: Vec<Article>,
    feed_input: String,
    loading: bool,
    settings: AppSettings,
    show_settings: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Sidebar(SidebarMessage),
    Content(ContentMessage),
    Settings(SettingsMessage),
    FeedFetched(String, Result<FeedData, String>),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                feeds: vec![],
                articles: vec![],
                feed_input: String::new(),
                loading: false,
                settings: AppSettings::default(),
                show_settings: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Sidebar(sidebar_msg) => match sidebar_msg {
                SidebarMessage::FeedInputChanged(value) => {
                    self.feed_input = value;
                    Task::none()
                }
                SidebarMessage::AddFeed => {
                    if self.feed_input.trim().is_empty() {
                        return Task::none();
                    }

                    let url = self.feed_input.clone();
                    self.feeds.push(Feed::new(url.clone(), "Loading...".to_string()));
                    self.feed_input.clear();
                    self.loading = true;

                    Task::perform(
                        async move {
                            let result = feed_engine::fetch_feed(url.clone()).await;
                            (url, result)
                        },
                        |(url, result)| Message::FeedFetched(url, result),
                    )
                }
                SidebarMessage::RefreshFeed(url) => {
                    self.loading = true;
                    Task::perform(
                        async move {
                            let result = feed_engine::fetch_feed(url.clone()).await;
                            (url, result)
                        },
                        |(url, result)| Message::FeedFetched(url, result),
                    )
                }
                SidebarMessage::OpenSettings => {
                    self.show_settings = true;
                    Task::none()
                }
            },
            Message::Content(_) => Task::none(),
            Message::Settings(settings_msg) => match settings_msg {
                SettingsMessage::ThemeSelected(theme) => {
                    self.settings.theme = theme;
                    Task::none()
                }
                SettingsMessage::ToggleImages => {
                    self.settings.show_images = !self.settings.show_images;
                    Task::none()
                }
                SettingsMessage::ToggleExcerpts => {
                    self.settings.show_excerpts = !self.settings.show_excerpts;
                    Task::none()
                }
                SettingsMessage::CloseSettings => {
                    self.show_settings = false;
                    Task::none()
                }
            },
            Message::FeedFetched(url, result) => {
                self.loading = false;
                match result {
                    Ok(feed_data) => {
                        // Update feed title
                        if let Some(feed) = self.feeds.iter_mut().find(|f| f.url == url) {
                            feed.title = feed_data.title;
                        }
                        self.articles = feed_data.articles;
                    }
                    Err(e) => {
                        eprintln!("Error fetching feed: {}", e);
                        // Remove the feed if it failed to load
                        self.feeds.retain(|f| f.url != url);
                    }
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        use iced::widget::{container, row};
        use iced::Length;

        if self.show_settings {
            ui::settings_view(&self.settings).map(Message::Settings)
        } else {
            let content = row![
                ui::sidebar_view(&self.feeds, &self.feed_input).map(Message::Sidebar),
                ui::content_view(&self.articles, self.loading, &self.settings)
                    .map(Message::Content),
            ]
            .spacing(0);

            container(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(0)
                .into()
        }
    }

    fn theme(&self) -> Theme {
        self.settings.theme.to_iced_theme()
    }
}

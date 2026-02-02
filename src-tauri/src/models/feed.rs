use serde::{Deserialize, Serialize};
use super::Article;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feed {
    pub url: String,
    pub title: String,
}

impl Feed {
    pub fn new(url: String, title: String) -> Self {
        Self { url, title }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedData {
    pub title: String,
    pub articles: Vec<Article>,
}

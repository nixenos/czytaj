use feed_rs::parser;

#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub link: String,
}

pub struct FeedEngine;

impl FeedEngine {
    pub async fn fetch_feed(url: String) -> Result<Vec<Article>, String> {
        // Fetch the RSS/Atom feed
        let response = reqwest::get(&url)
            .await
            .map_err(|e| format!("Failed to fetch feed: {}", e))?;

        let content = response
            .text()
            .await
            .map_err(|e| format!("Failed to read feed content: {}", e))?;

        // Parse the feed
        let feed = parser::parse(content.as_bytes())
            .map_err(|e| format!("Failed to parse feed: {}", e))?;

        // Extract articles
        let articles: Vec<Article> = feed
            .entries
            .iter()
            .map(|entry| {
                let title = entry
                    .title
                    .as_ref()
                    .map(|t| t.content.clone())
                    .unwrap_or_else(|| "Untitled".to_string());

                let link = entry
                    .links
                    .first()
                    .map(|l| l.href.clone())
                    .unwrap_or_else(|| "No link".to_string());

                Article { title, link }
            })
            .collect();

        Ok(articles)
    }
}

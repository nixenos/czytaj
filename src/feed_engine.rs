use feed_rs::parser;

#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub link: String,
}

#[derive(Debug, Clone)]
pub struct FeedData {
    pub title: String,
    pub articles: Vec<Article>,
}

pub async fn fetch_feed(url: String) -> Result<FeedData, String> {
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

    // Extract feed title
    let feed_title = feed
        .title
        .as_ref()
        .map(|t| t.content.clone())
        .unwrap_or_else(|| url.clone());

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

    Ok(FeedData {
        title: feed_title,
        articles,
    })
}

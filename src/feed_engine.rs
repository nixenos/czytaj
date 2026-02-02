use feed_rs::parser;
use crate::models::{Article, FeedData};
use crate::utils::sanitize_html;

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

            // Extract and sanitize excerpt from summary or content
            let excerpt = entry
                .summary
                .as_ref()
                .map(|text| sanitize_html(&text.content))
                .or_else(|| {
                    entry.content.as_ref().and_then(|c| {
                        c.body.as_ref().map(|body| sanitize_html(body))
                    })
                });

            // Extract image URL from media content or content
            let image_url = extract_image_url(entry);

            Article::new(title, link)
                .with_excerpt(excerpt)
                .with_image(image_url)
        })
        .collect();

    Ok(FeedData {
        title: feed_title,
        articles,
    })
}

fn extract_image_url(entry: &feed_rs::model::Entry) -> Option<String> {
    // Try to get image from media content
    if let Some(media) = entry.media.first() {
        if let Some(content) = media.content.first() {
            if let Some(url) = &content.url {
                return Some(url.to_string());
            }
        }
        if let Some(thumbnail) = media.thumbnails.first() {
            return Some(thumbnail.image.uri.clone());
        }
    }

    // Try to extract from content
    if let Some(content) = &entry.content {
        if let Some(body) = &content.body {
            // Simple regex to find first image in HTML
            if let Some(img_url) = extract_first_image_from_html(body) {
                return Some(img_url);
            }
        }
    }

    None
}

fn extract_first_image_from_html(html: &str) -> Option<String> {
    use regex::Regex;
    
    let img_regex = Regex::new(r#"<img[^>]+src=["']([^"']+)["']"#).ok()?;
    img_regex
        .captures(html)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

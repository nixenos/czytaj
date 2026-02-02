use std::sync::{Arc, Mutex};
use tauri::State;

mod db;
mod feed_engine;
mod models;
mod utils;

use db::ArticleDatabase;
use models::{AppSettings, Article, Feed, FeedData};

// Application state
pub struct AppState {
    db: Arc<Mutex<ArticleDatabase>>,
    settings: Arc<Mutex<AppSettings>>,
    feeds: Arc<Mutex<Vec<Feed>>>,
}

// Tauri commands
#[tauri::command]
async fn add_feed(url: String, state: State<'_, AppState>) -> Result<FeedData, String> {
    let feed_data = feed_engine::fetch_feed(url.clone()).await?;
    
    // Add feed to state
    let mut feeds = state.feeds.lock().unwrap();
    feeds.push(Feed::new(url, feed_data.title.clone()));
    
    Ok(feed_data)
}

#[tauri::command]
async fn refresh_feed(url: String) -> Result<FeedData, String> {
    feed_engine::fetch_feed(url).await
}

#[tauri::command]
async fn get_feeds(state: State<'_, AppState>) -> Result<Vec<Feed>, String> {
    let feeds = state.feeds.lock().unwrap();
    Ok(feeds.clone())
}

#[tauri::command]
async fn mark_article_viewed(url: String, title: String, state: State<'_, AppState>) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.mark_as_viewed(&url, &title)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn is_article_viewed(url: String, state: State<'_, AppState>) -> Result<bool, String> {
    let db = state.db.lock().unwrap();
    db.is_viewed(&url)
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_viewed_articles(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let db = state.db.lock().unwrap();
    db.get_viewed_articles()
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let settings = state.settings.lock().unwrap();
    Ok(settings.clone())
}

#[tauri::command]
async fn update_settings(new_settings: AppSettings, state: State<'_, AppState>) -> Result<(), String> {
    let mut settings = state.settings.lock().unwrap();
    *settings = new_settings;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize database
    let db = ArticleDatabase::new().expect("Failed to initialize database");
    
    // Create application state
    let state = AppState {
        db: Arc::new(Mutex::new(db)),
        settings: Arc::new(Mutex::new(AppSettings::default())),
        feeds: Arc::new(Mutex::new(Vec::new())),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            add_feed,
            refresh_feed,
            get_feeds,
            mark_article_viewed,
            is_article_viewed,
            get_viewed_articles,
            get_settings,
            update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

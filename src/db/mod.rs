use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Database manager for tracking viewed articles
pub struct ArticleDatabase {
    conn: Arc<Mutex<Connection>>,
}

impl ArticleDatabase {
    /// Create a new database connection and initialize tables
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path();
        let conn = Connection::open(db_path)?;
        
        // Create the viewed_articles table if it doesn't exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS viewed_articles (
                article_url TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                viewed_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;
        
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }
    
    /// Get the path to the database file
    fn get_db_path() -> PathBuf {
        // Use user's data directory or fallback to current directory
        let mut path = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."));
        path.push("czytaj");
        std::fs::create_dir_all(&path).ok();
        path.push("articles.db");
        path
    }
    
    /// Mark an article as viewed
    pub fn mark_as_viewed(&self, article_url: &str, title: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO viewed_articles (article_url, title, viewed_at) 
             VALUES (?1, ?2, CURRENT_TIMESTAMP)",
            [article_url, title],
        )?;
        Ok(())
    }
    
    /// Check if an article has been viewed
    pub fn is_viewed(&self, article_url: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT COUNT(*) FROM viewed_articles WHERE article_url = ?1"
        )?;
        let count: i64 = stmt.query_row([article_url], |row| row.get(0))?;
        Ok(count > 0)
    }
    
    /// Get all viewed article URLs
    pub fn get_viewed_articles(&self) -> Result<Vec<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT article_url FROM viewed_articles ORDER BY viewed_at DESC"
        )?;
        let articles = stmt
            .query_map([], |row| row.get(0))?
            .collect::<Result<Vec<String>>>()?;
        Ok(articles)
    }
}

impl Clone for ArticleDatabase {
    fn clone(&self) -> Self {
        Self {
            conn: Arc::clone(&self.conn),
        }
    }
}

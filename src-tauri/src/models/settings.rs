use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    Light,
    Dark,
}

impl std::fmt::Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl AppTheme {
    pub fn all() -> Vec<AppTheme> {
        vec![
            AppTheme::Light,
            AppTheme::Dark,
        ]
    }

    pub fn name(&self) -> &str {
        match self {
            AppTheme::Light => "Light",
            AppTheme::Dark => "Dark",
        }
    }
    
    pub fn is_dark(&self) -> bool {
        matches!(self, AppTheme::Dark)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: AppTheme,
    pub show_images: bool,
    pub show_excerpts: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: AppTheme::Light,
            show_images: true,
            show_excerpts: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Article {
    pub title: String,
    pub link: String,
    pub excerpt: Option<String>,
    pub image_url: Option<String>,
}

impl Article {
    pub fn new(title: String, link: String) -> Self {
        Self {
            title,
            link,
            excerpt: None,
            image_url: None,
        }
    }

    pub fn with_excerpt(mut self, excerpt: Option<String>) -> Self {
        self.excerpt = excerpt;
        self
    }

    pub fn with_image(mut self, image_url: Option<String>) -> Self {
        self.image_url = image_url;
        self
    }
}

use regex::Regex;
use once_cell::sync::Lazy;

// Compile regexes once for better performance
static SCRIPT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)<script[^>]*>.*?</script>").unwrap()
});

static STYLE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)<style[^>]*>.*?</style>").unwrap()
});

static IFRAME_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)<iframe[^>]*>.*?</iframe>").unwrap()
});

static EVENT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?i)\s+on\w+\s*=\s*["'][^"']*["']"#).unwrap()
});

static JS_PROTOCOL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)javascript:").unwrap()
});

static HTML_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"<[^>]+>").unwrap()
});

/// Sanitize HTML content by removing potentially dangerous tags and scripts
pub fn sanitize_html(content: &str) -> String {
    let mut sanitized = content.to_string();
    
    // Remove script tags and their content
    sanitized = SCRIPT_REGEX.replace_all(&sanitized, "").to_string();
    
    // Remove style tags and their content
    sanitized = STYLE_REGEX.replace_all(&sanitized, "").to_string();
    
    // Remove iframe tags
    sanitized = IFRAME_REGEX.replace_all(&sanitized, "").to_string();
    
    // Remove event handlers (onclick, onerror, etc.)
    sanitized = EVENT_REGEX.replace_all(&sanitized, "").to_string();
    
    // Remove javascript: protocol
    sanitized = JS_PROTOCOL_REGEX.replace_all(&sanitized, "").to_string();
    
    // Strip remaining HTML tags for plain text
    sanitized = HTML_REGEX.replace_all(&sanitized, "").to_string();
    
    // Decode HTML entities
    sanitized = html_escape::decode_html_entities(&sanitized).to_string();
    
    // Trim and limit length
    sanitized = sanitized.trim().chars().take(500).collect();
    
    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_removes_scripts() {
        let input = "Hello <script>alert('XSS')</script> World";
        let result = sanitize_html(input);
        assert!(!result.contains("script"));
        assert!(!result.contains("XSS"));
    }

    #[test]
    fn test_sanitize_removes_event_handlers() {
        let input = r#"<div onclick="alert('XSS')">Click me</div>"#;
        let result = sanitize_html(input);
        assert!(!result.contains("onclick"));
    }
}

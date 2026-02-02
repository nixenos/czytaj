use regex::Regex;

/// Sanitize HTML content by removing potentially dangerous tags and scripts
pub fn sanitize_html(content: &str) -> String {
    let mut sanitized = content.to_string();
    
    // Remove script tags and their content
    let script_regex = Regex::new(r"(?i)<script[^>]*>.*?</script>").unwrap();
    sanitized = script_regex.replace_all(&sanitized, "").to_string();
    
    // Remove style tags and their content
    let style_regex = Regex::new(r"(?i)<style[^>]*>.*?</style>").unwrap();
    sanitized = style_regex.replace_all(&sanitized, "").to_string();
    
    // Remove iframe tags
    let iframe_regex = Regex::new(r"(?i)<iframe[^>]*>.*?</iframe>").unwrap();
    sanitized = iframe_regex.replace_all(&sanitized, "").to_string();
    
    // Remove event handlers (onclick, onerror, etc.)
    let event_regex = Regex::new(r#"(?i)\s+on\w+\s*=\s*["'][^"']*["']"#).unwrap();
    sanitized = event_regex.replace_all(&sanitized, "").to_string();
    
    // Remove javascript: protocol
    let js_protocol_regex = Regex::new(r"(?i)javascript:").unwrap();
    sanitized = js_protocol_regex.replace_all(&sanitized, "").to_string();
    
    // Strip remaining HTML tags for plain text
    let html_regex = Regex::new(r"<[^>]+>").unwrap();
    sanitized = html_regex.replace_all(&sanitized, "").to_string();
    
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

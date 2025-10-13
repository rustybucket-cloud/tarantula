use regex::Regex;

pub fn is_url(s: &str) -> bool {
    let url_regex = Regex::new(r"^(http|https)://").unwrap();
    url_regex.is_match(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detects_url() {
        let url = "https://example.com";
        assert!(is_url(url));
    }

    #[test]
    fn detects_non_url() {
        let not_url = "example.com";
        assert!(!is_url(not_url));
    }
}

/// Estimate token count using a character-based approximation
/// This is a simplified version - in production, you'd use tiktoken or similar
pub fn estimate_tokens(text: &str) -> usize {
    if text.is_empty() {
        return 0;
    }

    // Simple estimation: English ~4 chars per token, Chinese ~1.5 chars per token
    let mut token_count = 0;
    let mut latin_chars = 0;
    let mut cjk_chars = 0;
    let mut other_chars = 0;

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch.is_ascii_punctuation() {
            latin_chars += 1;
        } else if ('\u{4e00}'..='\u{9fff}').contains(&ch) 
            || ('\u{3400}'..='\u{4dbf}').contains(&ch)
            || ('\u{3000}'..='\u{303f}').contains(&ch) {
            cjk_chars += 1;
        } else {
            other_chars += 1;
        }
    }

    // English/Latin: ~4 chars per token
    token_count += (latin_chars as f64 / 4.0).ceil() as usize;
    // CJK: ~1.5 chars per token
    token_count += (cjk_chars as f64 / 1.5).ceil() as usize;
    // Other: ~3 chars per token
    token_count += (other_chars as f64 / 3.0).ceil() as usize;
    
    token_count.max(1)
}

/// Estimate tokens for multiple texts
pub fn estimate_tokens_batch(texts: &[String]) -> Vec<usize> {
    texts.iter().map(|t| estimate_tokens(t)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_estimate_tokens_empty() {
        assert_eq!(estimate_tokens(""), 0);
    }

    #[test]
    fn test_estimate_tokens_simple() {
        let text = "Hello world";
        let tokens = estimate_tokens(text);
        assert!(tokens > 0);
        assert!(tokens <= text.len());
    }

    #[test]
    fn test_estimate_tokens_chinese() {
        let text = "你好世界";
        let tokens = estimate_tokens(text);
        // Chinese: ~1.5 chars per token, so 4 chars ~ 3 tokens
        assert!(tokens > 0);
        assert!(tokens <= text.chars().count() * 2);
    }


}

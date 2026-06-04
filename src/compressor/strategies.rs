use anyhow::{anyhow, Result};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionStrategy {
    Summary,
    Truncate,
    Semantic,
}

impl CompressionStrategy {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "summary" | "summarize" | "摘要" | "智能摘要" => Ok(CompressionStrategy::Summary),
            "truncate" | "truncation" | "截断" | "智能截断" => Ok(CompressionStrategy::Truncate),
            "semantic" | "semantics" | "语义" | "语义压缩" => Ok(CompressionStrategy::Semantic),
            _ => Err(anyhow!("Invalid compression strategy: {}", s)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CompressionStrategy::Summary => "summary",
            CompressionStrategy::Truncate => "truncate",
            CompressionStrategy::Semantic => "semantic",
        }
    }
}

lazy_static! {
    static ref SENTENCE_PATTERN: Regex = Regex::new(r"[.!?。！？]+\s*").unwrap();
    static ref PARAGRAPH_PATTERN: Regex = Regex::new(r"\n\s*\n").unwrap();
    static ref WHITESPACE_PATTERN: Regex = Regex::new(r"\s+").unwrap();
}

/// Smart summary compression - extract key sentences
pub fn summary_compress(text: &str, max_tokens: Option<usize>) -> Result<String> {
    if text.is_empty() {
        return Ok(String::new());
    }

    let paragraphs: Vec<&str> = text.split("\n\n").filter(|p| !p.trim().is_empty()).collect();
    
    if paragraphs.is_empty() {
        return Ok(text.to_string());
    }

    // Extract first sentence of each paragraph as summary
    let mut summary_parts = Vec::new();
    
    for (i, paragraph) in paragraphs.iter().enumerate() {
        let trimmed = paragraph.trim();
        if trimmed.is_empty() {
            continue;
        }

        // For first paragraph, include more content
        if i == 0 {
            let sentences: Vec<&str> = SENTENCE_PATTERN.split(trimmed).filter(|s| !s.is_empty()).collect();
            if sentences.len() >= 2 {
                summary_parts.push(format!("{}.", sentences[0].trim()));
                summary_parts.push(format!("{}.", sentences[1].trim()));
            } else {
                summary_parts.push(trimmed.to_string());
            }
        } else {
            // For other paragraphs, just take the first sentence
            let first_sentence = SENTENCE_PATTERN.split(trimmed).next().unwrap_or(trimmed);
            if !first_sentence.trim().is_empty() {
                summary_parts.push(format!("{}.", first_sentence.trim()));
            }
        }
    }

    let result = summary_parts.join(" ");
    
    // Apply max tokens limit if specified
    if let Some(max) = max_tokens {
        return apply_token_limit(&result, max);
    }

    Ok(result)
}

/// Smart truncation - keep beginning and end, remove middle
pub fn truncate_compress(text: &str, max_tokens: Option<usize>) -> Result<String> {
    if text.is_empty() {
        return Ok(String::new());
    }

    let target_tokens = max_tokens.unwrap_or(1000);
    let approx_chars_per_token = 4;
    let target_chars = target_tokens * approx_chars_per_token;

    if text.len() <= target_chars {
        return Ok(text.to_string());
    }

    // Keep 40% from beginning and 40% from end
    let keep_chars = target_chars * 4 / 10;
    let beginning = &text[..keep_chars.min(text.len())];
    
    let end_start = if text.len() > keep_chars {
        text.len() - keep_chars.min(text.len() - keep_chars)
    } else {
        keep_chars
    };
    let end = &text[end_start..];

    // Find sentence boundaries
    let beginning_clean = find_last_sentence_end(beginning);
    let end_clean = find_first_sentence_start(end);

    let result = format!("{}\n\n[... {} characters truncated ...]\n\n{}", 
        beginning_clean, 
        text.len() - beginning_clean.len() - end_clean.len(),
        end_clean
    );

    Ok(result)
}

/// Semantic compression - merge similar sentences and remove redundancy
pub fn semantic_compress(text: &str, max_tokens: Option<usize>) -> Result<String> {
    if text.is_empty() {
        return Ok(String::new());
    }

    // Step 1: Remove redundant whitespace
    let cleaned = WHITESPACE_PATTERN.replace_all(text, " ");
    
    // Step 2: Split into sentences and remove very similar ones
    let sentences: Vec<&str> = SENTENCE_PATTERN.split(&cleaned).filter(|s| !s.trim().is_empty()).collect();
    
    let mut unique_sentences = Vec::new();
    for sentence in sentences {
        let trimmed = sentence.trim();
        if trimmed.len() < 10 {
            continue; // Skip very short sentences
        }
        
        // Check if this sentence is too similar to any already kept
        let is_duplicate = unique_sentences.iter().any(|kept: &&str| {
            similarity(trimmed, kept) > 0.7
        });
        
        if !is_duplicate {
            unique_sentences.push(trimmed);
        }
    }

    let result = unique_sentences.join(". ");
    let result = if !result.ends_with('.') {
        format!("{}.", result)
    } else {
        result
    };

    // Apply max tokens limit if specified
    if let Some(max) = max_tokens {
        return apply_token_limit(&result, max);
    }

    Ok(result)
}

/// Apply a token limit by truncating
fn apply_token_limit(text: &str, max_tokens: usize) -> Result<String> {
    let approx_chars = max_tokens * 4;
    
    if text.len() <= approx_chars {
        return Ok(text.to_string());
    }

    // Try to find a good breaking point
    let truncated = &text[..approx_chars];
    let last_space = truncated.rfind(' ').unwrap_or(approx_chars);
    
    Ok(format!("{}...", &truncated[..last_space]))
}

/// Find the last complete sentence end
fn find_last_sentence_end(text: &str) -> &str {
    let mut last_end = text.len();
    
    for (i, _) in text.char_indices() {
        if i + 1 < text.len() {
            let ch = text.chars().nth(i).unwrap();
            if ch == '.' || ch == '!' || ch == '?' || ch == '。' || ch == '！' || ch == '？' {
                last_end = i + 1;
            }
        }
    }
    
    &text[..last_end]
}

/// Find the first complete sentence start
fn find_first_sentence_start(text: &str) -> &str {
    for (i, ch) in text.char_indices() {
        if ch.is_uppercase() || ch.is_numeric() {
            return &text[i..];
        }
    }
    
    text
}

/// Simple similarity metric between two strings (Jaccard-like)
fn similarity(a: &str, b: &str) -> f64 {
    let a_words: std::collections::HashSet<String> = a.to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let b_words: std::collections::HashSet<String> = b.to_lowercase()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    if a_words.is_empty() || b_words.is_empty() {
        return 0.0;
    }
    
    let intersection: std::collections::HashSet<_> = a_words.intersection(&b_words).collect();
    let union: std::collections::HashSet<_> = a_words.union(&b_words).collect();
    
    intersection.len() as f64 / union.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_compress() {
        let text = "First paragraph. Second sentence.\n\nSecond paragraph. Another sentence.\n\nThird paragraph. Final sentence.";
        let result = summary_compress(text, None).unwrap();
        assert!(!result.is_empty());
        assert!(result.len() < text.len());
    }

    #[test]
    fn test_truncate_compress() {
        let text = "A".repeat(10000);
        let result = truncate_compress(&text, Some(100)).unwrap();
        assert!(result.len() < text.len());
        assert!(result.contains("truncated"));
    }

    #[test]
    fn test_semantic_compress() {
        let text = "This is a test. This is a test. This is unique. Another sentence here.";
        let result = semantic_compress(text, None).unwrap();
        assert!(!result.is_empty());
    }

    #[test]
    fn test_strategy_from_str() {
        assert_eq!(CompressionStrategy::from_str("summary").unwrap(), CompressionStrategy::Summary);
        assert_eq!(CompressionStrategy::from_str("truncate").unwrap(), CompressionStrategy::Truncate);
        assert_eq!(CompressionStrategy::from_str("semantic").unwrap(), CompressionStrategy::Semantic);
        assert!(CompressionStrategy::from_str("invalid").is_err());
    }
}

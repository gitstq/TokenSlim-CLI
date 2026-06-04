pub mod strategies;
pub mod tokenizer;

use anyhow::Result;
use strategies::CompressionStrategy;
use tokenizer::estimate_tokens;

#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub original_text: String,
    pub compressed_text: String,
    pub original_tokens: usize,
    pub compressed_tokens: usize,
    pub strategy: CompressionStrategy,
    pub compression_ratio: f64,
    pub tokens_saved: usize,
}

impl CompressionResult {
    pub fn new(
        original_text: String,
        compressed_text: String,
        strategy: CompressionStrategy,
    ) -> Self {
        let original_tokens = estimate_tokens(&original_text);
        let compressed_tokens = estimate_tokens(&compressed_text);
        let tokens_saved = original_tokens.saturating_sub(compressed_tokens);
        let compression_ratio = if original_tokens > 0 {
            (tokens_saved as f64 / original_tokens as f64) * 100.0
        } else {
            0.0
        };

        Self {
            original_text,
            compressed_text,
            original_tokens,
            compressed_tokens,
            strategy,
            compression_ratio,
            tokens_saved,
        }
    }
}

pub fn compress(text: &str, strategy: CompressionStrategy, max_tokens: Option<usize>) -> Result<CompressionResult> {
    let compressed = match strategy {
        CompressionStrategy::Summary => strategies::summary_compress(text, max_tokens),
        CompressionStrategy::Truncate => strategies::truncate_compress(text, max_tokens),
        CompressionStrategy::Semantic => strategies::semantic_compress(text, max_tokens),
    }?;

    Ok(CompressionResult::new(text.to_string(), compressed, strategy))
}

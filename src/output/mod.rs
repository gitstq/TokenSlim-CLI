pub mod formats;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::compressor::CompressionResult;
use crate::models::pricing::CostComparison;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Json,
    Markdown,
    Text,
    Table,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "markdown" | "md" => Ok(OutputFormat::Markdown),
            "text" | "plain" | "txt" => Ok(OutputFormat::Text),
            "table" => Ok(OutputFormat::Table),
            _ => Err(anyhow::anyhow!("Invalid output format: {}", s)),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Json => "json",
            OutputFormat::Markdown => "markdown",
            OutputFormat::Text => "text",
            OutputFormat::Table => "table",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutputData {
    pub compressed_text: String,
    pub original_tokens: usize,
    pub compressed_tokens: usize,
    pub compression_ratio: f64,
    pub tokens_saved: usize,
    pub strategy: String,
    pub cost_comparisons: Vec<CostComparisonData>,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CostComparisonData {
    pub model_id: String,
    pub model_name: String,
    pub provider: String,
    pub original_cost_usd: f64,
    pub compressed_cost_usd: f64,
    pub savings_usd: f64,
    pub savings_percent: f64,
}

impl From<CostComparison> for CostComparisonData {
    fn from(c: CostComparison) -> Self {
        Self {
            model_id: c.model.id,
            model_name: c.model.name,
            provider: c.model.provider.to_string(),
            original_cost_usd: c.original_cost,
            compressed_cost_usd: c.compressed_cost,
            savings_usd: c.savings,
            savings_percent: c.savings_percent,
        }
    }
}

pub fn build_output_data(
    result: &CompressionResult,
    cost_comparisons: Vec<CostComparison>,
) -> OutputData {
    OutputData {
        compressed_text: result.compressed_text.clone(),
        original_tokens: result.original_tokens,
        compressed_tokens: result.compressed_tokens,
        compression_ratio: result.compression_ratio,
        tokens_saved: result.tokens_saved,
        strategy: result.strategy.as_str().to_string(),
        cost_comparisons: cost_comparisons.into_iter().map(Into::into).collect(),
        timestamp: chrono::Local::now().to_rfc3339(),
    }
}

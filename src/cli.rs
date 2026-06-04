use clap::{Parser, ValueEnum};
use std::path::PathBuf;

use crate::compressor::strategies::CompressionStrategy;
use crate::i18n::Language;
use crate::output::OutputFormat;

#[derive(Parser, Debug)]
#[command(
    name = "tokenslim",
    about = "🚀 TokenSlim-CLI - High-performance LLM Token Compression & Cost Estimation",
    version = env!("CARGO_PKG_VERSION"),
    author = "TokenSlim Team",
    long_about = r#"
TokenSlim-CLI is a high-performance command-line tool for compressing text 
to reduce LLM token usage and estimating API costs across 20+ models.

EXAMPLES:
    tokenslim -i input.txt -s summary                    # Compress with summary strategy
    tokenslim -i input.txt -o output.md -f markdown      # Output as markdown
    tokenslim --wizard                                   # Interactive configuration wizard
    cat input.txt | tokenslim -s semantic                # Pipe input
    tokenslim -i "*.txt" -s truncate --max-tokens 500   # Batch process with glob

STRATEGIES:
    summary   - Extract key sentences from each paragraph
    truncate  - Keep beginning and end, truncate middle
    semantic  - Remove redundant and similar sentences
    "#
)]
pub struct Args {
    /// Input file path (supports glob patterns like *.txt)
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<String>,

    /// Output file path (default: stdout)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Compression strategy
    #[arg(short, long, value_enum, default_value = "summary")]
    pub strategy: CliStrategy,

    /// Maximum tokens limit for output
    #[arg(long, value_name = "TOKENS")]
    pub max_tokens: Option<usize>,

    /// Output format
    #[arg(short, long, value_enum, default_value = "table")]
    pub format: CliFormat,

    /// LLM model for cost estimation
    #[arg(short, long, default_value = "gpt-4o-mini")]
    pub model: String,

    /// Interface language
    #[arg(short, long, value_enum, default_value = "en")]
    pub language: CliLanguage,

    /// Run interactive configuration wizard
    #[arg(long)]
    pub wizard: bool,

    /// Show only compressed text without stats
    #[arg(long)]
    pub quiet: bool,

    /// Include full compressed text in output
    #[arg(long)]
    pub include_text: bool,

    /// Show all model cost comparisons
    #[arg(long)]
    pub all_models: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliStrategy {
    Summary,
    Truncate,
    Semantic,
}

impl From<CliStrategy> for CompressionStrategy {
    fn from(s: CliStrategy) -> Self {
        match s {
            CliStrategy::Summary => CompressionStrategy::Summary,
            CliStrategy::Truncate => CompressionStrategy::Truncate,
            CliStrategy::Semantic => CompressionStrategy::Semantic,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliFormat {
    Json,
    Markdown,
    Text,
    Table,
}

impl From<CliFormat> for OutputFormat {
    fn from(f: CliFormat) -> Self {
        match f {
            CliFormat::Json => OutputFormat::Json,
            CliFormat::Markdown => OutputFormat::Markdown,
            CliFormat::Text => OutputFormat::Text,
            CliFormat::Table => OutputFormat::Table,
        }
    }
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum CliLanguage {
    En,
    ZhCN,
    ZhTW,
}

impl From<CliLanguage> for Language {
    fn from(l: CliLanguage) -> Self {
        match l {
            CliLanguage::En => Language::En,
            CliLanguage::ZhCN => Language::ZhCN,
            CliLanguage::ZhTW => Language::ZhTW,
        }
    }
}

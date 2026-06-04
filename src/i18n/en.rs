use super::I18n;

pub struct En;

impl I18n for En {
    fn app_name(&self) -> &str {
        "🚀 TokenSlim-CLI"
    }

    fn app_description(&self) -> &str {
        "High-performance LLM Token Compression & Cost Estimation Tool"
    }

    fn version_info(&self, version: &str) -> String {
        format!("{} v{}", self.app_name(), version)
    }

    fn compress_success(&self, original: usize, compressed: usize, ratio: f64) -> String {
        format!(
            "✅ Compression complete! {} tokens → {} tokens ({:.1}% reduction)",
            original, compressed, ratio
        )
    }

    fn compress_strategy(&self) -> &str {
        "Compression Strategy"
    }

    fn strategy_summary(&self) -> &str {
        "📝 Smart Summary - Preserve key information, generate concise summary"
    }

    fn strategy_truncate(&self) -> &str {
        "✂️ Smart Truncation - Keep beginning and end, remove middle redundancy"
    }

    fn strategy_semantic(&self) -> &str {
        "🧠 Semantic Compression - Based on semantic analysis, intelligently merge similar content"
    }

    fn token_count(&self) -> &str {
        "Token Count"
    }

    fn cost_estimate(&self) -> &str {
        "💰 Cost Estimate"
    }

    fn model_name(&self) -> &str {
        "Model Name"
    }

    fn input_cost(&self) -> &str {
        "Input Cost"
    }

    fn output_cost(&self) -> &str {
        "Output Cost"
    }

    fn total_cost(&self) -> &str {
        "Total Cost"
    }

    fn savings(&self) -> &str {
        "Savings"
    }

    fn file_processed(&self) -> &str {
        "📄 File processed"
    }

    fn files_processed(&self, count: usize) -> String {
        format!("📁 {} files processed", count)
    }

    fn error_read_file(&self, path: &str) -> String {
        format!("❌ Cannot read file: {}", path)
    }

    fn error_invalid_strategy(&self) -> &str {
        "❌ Invalid compression strategy"
    }

    fn error_no_input(&self) -> &str {
        "❌ Please provide input file or use pipe input"
    }

    fn prompt_select_strategy(&self) -> &str {
        "Select compression strategy"
    }

    fn prompt_select_model(&self) -> &str {
        "Select LLM model"
    }

    fn prompt_select_language(&self) -> &str {
        "Select interface language"
    }

    fn prompt_max_tokens(&self) -> &str {
        "Enter maximum token limit"
    }

    fn prompt_input_file(&self) -> &str {
        "Enter input file path"
    }

    fn prompt_output_file(&self) -> &str {
        "Enter output file path (leave empty for console output)"
    }

    fn wizard_title(&self) -> &str {
        "🎯 TokenSlim Configuration Wizard"
    }

    fn wizard_complete(&self) -> &str {
        "🎉 Configuration complete! Running compression..."
    }

    fn format_json(&self) -> &str {
        "📊 JSON Format"
    }

    fn format_markdown(&self) -> &str {
        "📝 Markdown Format"
    }

    fn format_text(&self) -> &str {
        "📄 Plain Text Format"
    }

    fn format_table(&self) -> &str {
        "📋 Table Format"
    }

    fn option_yes(&self) -> &str {
        "Yes"
    }

    fn option_no(&self) -> &str {
        "No"
    }

    fn stats_original_tokens(&self) -> &str {
        "Original Tokens"
    }

    fn stats_compressed_tokens(&self) -> &str {
        "Compressed Tokens"
    }

    fn stats_compression_ratio(&self) -> &str {
        "Compression Ratio"
    }

    fn stats_estimated_cost(&self) -> &str {
        "Estimated Cost"
    }

    fn stats_savings_percent(&self) -> &str {
        "Savings %"
    }
}

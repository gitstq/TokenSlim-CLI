use super::I18n;

pub struct ZhTW;

impl I18n for ZhTW {
    fn app_name(&self) -> &str {
        "🚀 TokenSlim-CLI"
    }

    fn app_description(&self) -> &str {
        "高效能LLM Token壓縮與成本估算工具"
    }

    fn version_info(&self, version: &str) -> String {
        format!("{} 版本 {}", self.app_name(), version)
    }

    fn compress_success(&self, original: usize, compressed: usize, ratio: f64) -> String {
        format!(
            "✅ 壓縮完成！原始 {} tokens → 壓縮後 {} tokens（減少 {:.1}%）",
            original, compressed, ratio
        )
    }

    fn compress_strategy(&self) -> &str {
        "壓縮策略"
    }

    fn strategy_summary(&self) -> &str {
        "📝 智慧摘要 - 保留關鍵資訊，產生簡潔摘要"
    }

    fn strategy_truncate(&self) -> &str {
        "✂️ 智慧截斷 - 保留開頭和結尾，移除中間冗餘內容"
    }

    fn strategy_semantic(&self) -> &str {
        "🧠 語意壓縮 - 基於語意分析，智慧合併相似內容"
    }

    fn token_count(&self) -> &str {
        "Token數量"
    }

    fn cost_estimate(&self) -> &str {
        "💰 成本估算"
    }

    fn model_name(&self) -> &str {
        "模型名稱"
    }

    fn input_cost(&self) -> &str {
        "輸入成本"
    }

    fn output_cost(&self) -> &str {
        "輸出成本"
    }

    fn total_cost(&self) -> &str {
        "總成本"
    }

    fn savings(&self) -> &str {
        "節省金額"
    }

    fn file_processed(&self) -> &str {
        "📄 檔案已處理"
    }

    fn files_processed(&self, count: usize) -> String {
        format!("📁 已處理 {} 個檔案", count)
    }

    fn error_read_file(&self, path: &str) -> String {
        format!("❌ 無法讀取檔案: {}", path)
    }

    fn error_invalid_strategy(&self) -> &str {
        "❌ 無效的壓縮策略"
    }

    fn error_no_input(&self) -> &str {
        "❌ 請提供輸入檔案或使用管道輸入"
    }

    fn prompt_select_strategy(&self) -> &str {
        "請選擇壓縮策略"
    }

    fn prompt_select_model(&self) -> &str {
        "請選擇LLM模型"
    }

    fn prompt_select_language(&self) -> &str {
        "請選擇介面語言"
    }

    fn prompt_max_tokens(&self) -> &str {
        "請輸入最大Token限制"
    }

    fn prompt_input_file(&self) -> &str {
        "請輸入輸入檔案路徑"
    }

    fn prompt_output_file(&self) -> &str {
        "請輸入輸出檔案路徑（留空則輸出到控制台）"
    }

    fn wizard_title(&self) -> &str {
        "🎯 TokenSlim 配置精靈"
    }

    fn wizard_complete(&self) -> &str {
        "🎉 配置完成！正在執行壓縮..."
    }

    fn format_json(&self) -> &str {
        "📊 JSON格式"
    }

    fn format_markdown(&self) -> &str {
        "📝 Markdown格式"
    }

    fn format_text(&self) -> &str {
        "📄 純文字格式"
    }

    fn format_table(&self) -> &str {
        "📋 表格格式"
    }

    fn option_yes(&self) -> &str {
        "是"
    }

    fn option_no(&self) -> &str {
        "否"
    }

    fn stats_original_tokens(&self) -> &str {
        "原始Tokens"
    }

    fn stats_compressed_tokens(&self) -> &str {
        "壓縮後Tokens"
    }

    fn stats_compression_ratio(&self) -> &str {
        "壓縮率"
    }

    fn stats_estimated_cost(&self) -> &str {
        "預估成本"
    }

    fn stats_savings_percent(&self) -> &str {
        "節省比例"
    }
}

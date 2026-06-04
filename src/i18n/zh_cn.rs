use super::I18n;

pub struct ZhCN;

impl I18n for ZhCN {
    fn app_name(&self) -> &str {
        "🚀 TokenSlim-CLI"
    }

    fn app_description(&self) -> &str {
        "高性能LLM Token压缩与成本估算工具"
    }

    fn version_info(&self, version: &str) -> String {
        format!("{} 版本 {}", self.app_name(), version)
    }

    fn compress_success(&self, original: usize, compressed: usize, ratio: f64) -> String {
        format!(
            "✅ 压缩完成！原始 {} tokens → 压缩后 {} tokens（减少 {:.1}%）",
            original, compressed, ratio
        )
    }

    fn compress_strategy(&self) -> &str {
        "压缩策略"
    }

    fn strategy_summary(&self) -> &str {
        "📝 智能摘要 - 保留关键信息，生成简洁摘要"
    }

    fn strategy_truncate(&self) -> &str {
        "✂️ 智能截断 - 保留开头和结尾，移除中间冗余内容"
    }

    fn strategy_semantic(&self) -> &str {
        "🧠 语义压缩 - 基于语义分析，智能合并相似内容"
    }

    fn token_count(&self) -> &str {
        "Token数量"
    }

    fn cost_estimate(&self) -> &str {
        "💰 成本估算"
    }

    fn model_name(&self) -> &str {
        "模型名称"
    }

    fn input_cost(&self) -> &str {
        "输入成本"
    }

    fn output_cost(&self) -> &str {
        "输出成本"
    }

    fn total_cost(&self) -> &str {
        "总成本"
    }

    fn savings(&self) -> &str {
        "节省金额"
    }

    fn file_processed(&self) -> &str {
        "📄 文件已处理"
    }

    fn files_processed(&self, count: usize) -> String {
        format!("📁 已处理 {} 个文件", count)
    }

    fn error_read_file(&self, path: &str) -> String {
        format!("❌ 无法读取文件: {}", path)
    }

    fn error_invalid_strategy(&self) -> &str {
        "❌ 无效的压缩策略"
    }

    fn error_no_input(&self) -> &str {
        "❌ 请提供输入文件或使用管道输入"
    }

    fn prompt_select_strategy(&self) -> &str {
        "请选择压缩策略"
    }

    fn prompt_select_model(&self) -> &str {
        "请选择LLM模型"
    }

    fn prompt_select_language(&self) -> &str {
        "请选择界面语言"
    }

    fn prompt_max_tokens(&self) -> &str {
        "请输入最大Token限制"
    }

    fn prompt_input_file(&self) -> &str {
        "请输入输入文件路径"
    }

    fn prompt_output_file(&self) -> &str {
        "请输入输出文件路径（留空则输出到控制台）"
    }

    fn wizard_title(&self) -> &str {
        "🎯 TokenSlim 配置向导"
    }

    fn wizard_complete(&self) -> &str {
        "🎉 配置完成！正在执行压缩..."
    }

    fn format_json(&self) -> &str {
        "📊 JSON格式"
    }

    fn format_markdown(&self) -> &str {
        "📝 Markdown格式"
    }

    fn format_text(&self) -> &str {
        "📄 纯文本格式"
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
        "压缩后Tokens"
    }

    fn stats_compression_ratio(&self) -> &str {
        "压缩率"
    }

    fn stats_estimated_cost(&self) -> &str {
        "预估成本"
    }

    fn stats_savings_percent(&self) -> &str {
        "节省比例"
    }
}

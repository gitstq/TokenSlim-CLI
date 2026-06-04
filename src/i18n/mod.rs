use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    ZhCN,
    ZhTW,
    En,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Language::ZhCN => write!(f, "zh-CN"),
            Language::ZhTW => write!(f, "zh-TW"),
            Language::En => write!(f, "en"),
        }
    }
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "zh-cn" | "zh_cn" | "zh" | "cn" | "简体中文" | "简中" => Language::ZhCN,
            "zh-tw" | "zh_tw" | "tw" | "繁体中文" | "繁中" => Language::ZhTW,
            _ => Language::En,
        }
    }
}

pub trait I18n {
    fn app_name(&self) -> &str;
    fn app_description(&self) -> &str;
    fn version_info(&self, version: &str) -> String;
    fn compress_success(&self, original: usize, compressed: usize, ratio: f64) -> String;
    fn compress_strategy(&self) -> &str;
    fn strategy_summary(&self) -> &str;
    fn strategy_truncate(&self) -> &str;
    fn strategy_semantic(&self) -> &str;
    fn token_count(&self) -> &str;
    fn cost_estimate(&self) -> &str;
    fn model_name(&self) -> &str;
    fn input_cost(&self) -> &str;
    fn output_cost(&self) -> &str;
    fn total_cost(&self) -> &str;
    fn savings(&self) -> &str;
    fn file_processed(&self) -> &str;
    fn files_processed(&self, count: usize) -> String;
    fn error_read_file(&self, path: &str) -> String;
    fn error_invalid_strategy(&self) -> &str;
    fn error_no_input(&self) -> &str;
    fn prompt_select_strategy(&self) -> &str;
    fn prompt_select_model(&self) -> &str;
    fn prompt_select_language(&self) -> &str;
    fn prompt_max_tokens(&self) -> &str;
    fn prompt_input_file(&self) -> &str;
    fn prompt_output_file(&self) -> &str;
    fn wizard_title(&self) -> &str;
    fn wizard_complete(&self) -> &str;
    fn format_json(&self) -> &str;
    fn format_markdown(&self) -> &str;
    fn format_text(&self) -> &str;
    fn format_table(&self) -> &str;
    fn option_yes(&self) -> &str;
    fn option_no(&self) -> &str;
    fn stats_original_tokens(&self) -> &str;
    fn stats_compressed_tokens(&self) -> &str;
    fn stats_compression_ratio(&self) -> &str;
    fn stats_estimated_cost(&self) -> &str;
    fn stats_savings_percent(&self) -> &str;
}

pub fn get_i18n(lang: Language) -> Box<dyn I18n> {
    match lang {
        Language::ZhCN => Box::new(zh_cn::ZhCN),
        Language::ZhTW => Box::new(zh_tw::ZhTW),
        Language::En => Box::new(en::En),
    }
}

pub mod zh_cn;
pub mod zh_tw;
pub mod en;

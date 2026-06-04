pub mod wizard;

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use colored::Colorize;

use crate::i18n::Language;
use crate::compressor::strategies::CompressionStrategy;
use crate::models::get_all_models;
use crate::output::OutputFormat;

#[derive(Debug, Clone)]
pub struct WizardConfig {
    pub language: Language,
    pub strategy: CompressionStrategy,
    pub model_id: String,
    pub max_tokens: Option<usize>,
    pub input_file: Option<String>,
    pub output_file: Option<String>,
    pub output_format: OutputFormat,
}

impl Default for WizardConfig {
    fn default() -> Self {
        Self {
            language: Language::En,
            strategy: CompressionStrategy::Summary,
            model_id: "gpt-4o-mini".to_string(),
            max_tokens: None,
            input_file: None,
            output_file: None,
            output_format: OutputFormat::Table,
        }
    }
}

pub fn run_wizard() -> Result<WizardConfig> {
    let theme = ColorfulTheme::default();
    let mut config = WizardConfig::default();
    
    println!("{}", "🎯 TokenSlim Configuration Wizard".cyan().bold());
    println!("{}", "================================".cyan());
    
    // Step 1: Select language
    let languages = vec!["English", "简体中文", "繁體中文"];
    let lang_idx = Select::with_theme(&theme)
        .with_prompt("Select language / 选择语言")
        .items(&languages)
        .default(0)
        .interact()?;
    
    config.language = match lang_idx {
        1 => Language::ZhCN,
        2 => Language::ZhTW,
        _ => Language::En,
    };
    
    let i18n = crate::i18n::get_i18n(config.language);
    
    println!("\n{}", i18n.wizard_title().cyan().bold());
    
    // Step 2: Select strategy
    let strategies = vec![
        i18n.strategy_summary(),
        i18n.strategy_truncate(),
        i18n.strategy_semantic(),
    ];
    let strat_idx = Select::with_theme(&theme)
        .with_prompt(i18n.prompt_select_strategy())
        .items(&strategies)
        .default(0)
        .interact()?;
    
    config.strategy = match strat_idx {
        1 => CompressionStrategy::Truncate,
        2 => CompressionStrategy::Semantic,
        _ => CompressionStrategy::Summary,
    };
    
    // Step 3: Select model
    let models = get_all_models();
    let model_names: Vec<String> = models.iter()
        .map(|m| format!("{} - {} (${}/1M tokens)", m.name, m.provider, m.input_price_per_1m))
        .collect();
    
    let model_idx = Select::with_theme(&theme)
        .with_prompt(i18n.prompt_select_model())
        .items(&model_names)
        .default(1) // Default to GPT-4o Mini
        .interact()?;
    
    config.model_id = models[model_idx].id.clone();
    
    // Step 4: Max tokens limit
    let use_max_tokens = Confirm::with_theme(&theme)
        .with_prompt("Set maximum token limit?")
        .default(false)
        .interact()?;
    
    if use_max_tokens {
        let max: usize = Input::with_theme(&theme)
            .with_prompt(i18n.prompt_max_tokens())
            .default(1000)
            .interact()?;
        config.max_tokens = Some(max);
    }
    
    // Step 5: Output format
    let formats = vec![
        i18n.format_table(),
        i18n.format_json(),
        i18n.format_markdown(),
        i18n.format_text(),
    ];
    let format_idx = Select::with_theme(&theme)
        .with_prompt("Select output format")
        .items(&formats)
        .default(0)
        .interact()?;
    
    config.output_format = match format_idx {
        1 => OutputFormat::Json,
        2 => OutputFormat::Markdown,
        3 => OutputFormat::Text,
        _ => OutputFormat::Table,
    };
    
    // Step 6: Input file
    let use_file = Confirm::with_theme(&theme)
        .with_prompt("Read from file? (No = use stdin)")
        .default(true)
        .interact()?;
    
    if use_file {
        let input: String = Input::with_theme(&theme)
            .with_prompt(i18n.prompt_input_file())
            .interact_text()?;
        config.input_file = Some(input);
    }
    
    // Step 7: Output file
    let use_output = Confirm::with_theme(&theme)
        .with_prompt("Save to file?")
        .default(false)
        .interact()?;
    
    if use_output {
        let output: String = Input::with_theme(&theme)
            .with_prompt(i18n.prompt_output_file())
            .allow_empty(true)
            .interact_text()?;
        if !output.is_empty() {
            config.output_file = Some(output);
        }
    }
    
    println!("\n{}", i18n.wizard_complete().green().bold());
    
    Ok(config)
}

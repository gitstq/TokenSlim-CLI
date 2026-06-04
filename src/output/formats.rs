use anyhow::Result;
use colored::Colorize;
use tabled::{Table, Tabled, settings::Style};

use crate::compressor::CompressionResult;
use crate::i18n::I18n;
use crate::models::pricing::{CostComparison, format_usd};
use super::{OutputData, OutputFormat};

pub fn format_output(
    data: &OutputData,
    format: OutputFormat,
    i18n: &dyn I18n,
    include_text: bool,
) -> Result<String> {
    match format {
        OutputFormat::Json => format_json(data),
        OutputFormat::Markdown => format_markdown(data, i18n, include_text),
        OutputFormat::Text => format_text(data, i18n, include_text),
        OutputFormat::Table => format_table(data, i18n),
    }
}

fn format_json(data: &OutputData) -> Result<String> {
    Ok(serde_json::to_string_pretty(data)?)
}

fn format_markdown(data: &OutputData, i18n: &dyn I18n, include_text: bool) -> Result<String> {
    let mut output = String::new();
    
    output.push_str(&format!("# {}\n\n", i18n.app_name()));
    output.push_str(&format!("**{}**: {}\n\n", i18n.compress_strategy(), data.strategy));
    
    // Stats table
    output.push_str("## Stats\n\n");
    output.push_str(&format!("| {} | {} |\n", i18n.stats_original_tokens(), data.original_tokens));
    output.push_str(&format!("| {} | {} |\n", i18n.stats_compressed_tokens(), data.compressed_tokens));
    output.push_str(&format!("| {} | {:.1}% |\n", i18n.stats_compression_ratio(), data.compression_ratio));
    output.push_str(&format!("| {} | {} |\n\n", i18n.stats_savings_percent(), data.tokens_saved));
    
    // Cost comparison
    output.push_str(&format!("## {}\n\n", i18n.cost_estimate()));
    output.push_str(&format!("| {} | {} | {} | {} | {} |\n", 
        i18n.model_name(), i18n.input_cost(), i18n.total_cost(), i18n.savings(), "%"));
    output.push_str("|---|---|---|---|---|\n");
    
    for cost in &data.cost_comparisons[..5.min(data.cost_comparisons.len())] {
        output.push_str(&format!(
            "| {} | {} | {} | {} | {:.1}% |\n",
            cost.model_name,
            format_usd(cost.original_cost_usd),
            format_usd(cost.compressed_cost_usd),
            format_usd(cost.savings_usd),
            cost.savings_percent
        ));
    }
    
    if include_text {
        output.push_str(&format!("\n## Compressed Text\n\n```\n{}\n```\n", data.compressed_text));
    }
    
    Ok(output)
}

fn format_text(data: &OutputData, i18n: &dyn I18n, include_text: bool) -> Result<String> {
    let mut output = String::new();
    
    output.push_str(&format!("{}", i18n.compress_success(
        data.original_tokens,
        data.compressed_tokens,
        data.compression_ratio
    )));
    output.push('\n');
    
    if include_text {
        output.push_str("\n--- Compressed Text ---\n");
        output.push_str(&data.compressed_text);
        output.push_str("\n--- End ---\n");
    }
    
    Ok(output)
}

#[derive(Tabled)]
struct CostRow {
    #[tabled(rename = "Model")]
    model: String,
    #[tabled(rename = "Original")]
    original: String,
    #[tabled(rename = "Compressed")]
    compressed: String,
    #[tabled(rename = "Savings")]
    savings: String,
    #[tabled(rename = "Savings %")]
    savings_pct: String,
}

fn format_table(data: &OutputData, i18n: &dyn I18n) -> Result<String> {
    let mut output = String::new();
    
    output.push_str(&format!("{}\n\n", i18n.app_name().cyan().bold()));
    output.push_str(&format!("{}: {} → {} tokens ({:.1}% {} )\n\n",
        i18n.token_count().bold(),
        data.original_tokens,
        data.compressed_tokens,
        data.compression_ratio,
        i18n.savings()
    ));
    
    let rows: Vec<CostRow> = data.cost_comparisons[..10.min(data.cost_comparisons.len())]
        .iter()
        .map(|c| CostRow {
            model: format!("{} ({})", c.model_name, c.provider),
            original: format_usd(c.original_cost_usd),
            compressed: format_usd(c.compressed_cost_usd),
            savings: format_usd(c.savings_usd).green().to_string(),
            savings_pct: format!("{:.1}%", c.savings_percent),
        })
        .collect();
    
    let table = Table::new(rows).with(Style::modern()).to_string();
    output.push_str(&format!("{}\n", i18n.cost_estimate().bold()));
    output.push_str(&table);
    
    Ok(output)
}

pub fn print_quick_stats(result: &CompressionResult, i18n: &dyn I18n) {
    println!("{}", i18n.compress_success(
        result.original_tokens,
        result.compressed_tokens,
        result.compression_ratio
    ).green());
}

pub fn print_cost_table(comparisons: &[CostComparison], i18n: &dyn I18n) {
    println!("\n{}", i18n.cost_estimate().bold().cyan());
    
    #[derive(Tabled)]
    struct TableRow {
        #[tabled(rename = "Model")]
        model: String,
        #[tabled(rename = "Original Cost")]
        original: String,
        #[tabled(rename = "Compressed")]
        compressed: String,
        #[tabled(rename = "Savings")]
        savings: String,
        #[tabled(rename = "Saved %")]
        saved_pct: String,
    }
    
    let rows: Vec<TableRow> = comparisons[..10.min(comparisons.len())]
        .iter()
        .map(|c| TableRow {
            model: c.model.name.clone(),
            original: format_usd(c.original_cost),
            compressed: format_usd(c.compressed_cost),
            savings: format_usd(c.savings).green().to_string(),
            saved_pct: format!("{:.1}%", c.savings_percent),
        })
        .collect();
    
    let table = Table::new(rows).with(Style::modern()).to_string();
    println!("{}", table);
}

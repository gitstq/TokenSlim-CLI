use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use std::fs;
use std::io::{self, Read};

mod cli;
mod compressor;
mod i18n;
mod models;
mod output;
mod tui;

use cli::Args;
use compressor::compress;
use i18n::get_i18n;
use models::pricing::compare_costs;
use output::{build_output_data, formats, OutputFormat};
use tui::{run_wizard, WizardConfig};

fn main() -> Result<()> {
    // Enable colored output on Windows
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).ok();

    let args = Args::parse();

    // Run wizard if requested
    let config = if args.wizard {
        run_wizard()?
    } else {
        WizardConfig {
            language: args.language.into(),
            strategy: args.strategy.into(),
            model_id: args.model.clone(),
            max_tokens: args.max_tokens,
            input_file: args.input.clone(),
            output_file: args.output.as_ref().map(|p| p.to_string_lossy().to_string()),
            output_format: args.format.into(),
        }
    };

    let i18n = get_i18n(config.language);

    // Print app header
    if !args.quiet {
        println!("{}", i18n.app_name().cyan().bold());
        println!("{}", i18n.app_description().dimmed());
        println!();
    }

    // Read input
    let input_text = read_input(&config, &*i18n)?;

    if input_text.trim().is_empty() {
        return Err(anyhow::anyhow!("No input provided. Use -i for file input or pipe text via stdin."));
    }

    // Compress
    let result = compress(&input_text, config.strategy, config.max_tokens)
        .context("Compression failed")?;

    // Calculate costs
    let output_tokens_estimate = result.compressed_tokens / 2; // Rough estimate
    let cost_comparisons = if args.all_models {
        compare_costs(result.original_tokens, result.compressed_tokens, output_tokens_estimate)
    } else {
        let mut comparisons = compare_costs(result.original_tokens, result.compressed_tokens, output_tokens_estimate);
        comparisons.truncate(5);
        comparisons
    };

    // Build output data
    let output_data = build_output_data(&result, cost_comparisons.clone());

    // Format and output
    if args.quiet {
        println!("{}", result.compressed_text);
    } else {
        // Print quick stats
        formats::print_quick_stats(&result, &*i18n);

        // Print cost comparison table
        if !cost_comparisons.is_empty() {
            formats::print_cost_table(&cost_comparisons, &*i18n);
        }

        // Print formatted output if not table format (already printed above)
        if config.output_format != OutputFormat::Table {
            let formatted = formats::format_output(
                &output_data,
                config.output_format,
                &*i18n,
                args.include_text,
            )?;
            println!("\n{}", formatted);
        } else if args.include_text {
            println!("\n{}", "--- Compressed Text ---".dimmed());
            println!("{}", result.compressed_text);
        }
    }

    // Write to file if specified
    if let Some(output_path) = &config.output_file {
        let formatted = formats::format_output(
            &output_data,
            config.output_format,
            &*i18n,
            true,
        )?;
        fs::write(output_path, formatted)
            .with_context(|| format!("Failed to write to {}", output_path))?;
        
        if !args.quiet {
            println!("\n{} {}", "💾 Output saved to:".green(), output_path);
        }
    }

    Ok(())
}

fn read_input(config: &WizardConfig, i18n: &dyn i18n::I18n) -> Result<String> {
    match &config.input_file {
        Some(input) => {
            // Check if it's a glob pattern
            if input.contains('*') || input.contains('?') {
                read_glob_input(input, i18n)
            } else {
                fs::read_to_string(input)
                    .with_context(|| i18n.error_read_file(input))
            }
        }
        None => {
            // Read from stdin
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
    }
}

fn read_glob_input(pattern: &str, i18n: &dyn i18n::I18n) -> Result<String> {
    use glob::glob;
    
    let mut combined = String::new();
    let mut count = 0;
    
    for entry in glob(pattern)? {
        match entry {
            Ok(path) if path.is_file() => {
                match fs::read_to_string(&path) {
                    Ok(content) => {
                        combined.push_str(&format!("\n\n=== {} ===\n\n", path.display()));
                        combined.push_str(&content);
                        count += 1;
                    }
                    Err(_) => eprintln!("{}", i18n.error_read_file(&path.to_string_lossy())),
                }
            }
            _ => {}
        }
    }
    
    if count == 0 {
        return Err(anyhow::anyhow!("No files matched pattern: {}", pattern));
    }
    
    if !combined.is_empty() {
        println!("{}", i18n.files_processed(count));
    }
    
    Ok(combined)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_basic() {
        let text = "This is a test paragraph. It has multiple sentences.\n\nThis is another paragraph. It also has sentences.";
        let result = compress(text, compressor::strategies::CompressionStrategy::Summary, None).unwrap();
        assert!(result.compressed_tokens < result.original_tokens);
        assert!(result.compression_ratio > 0.0);
    }

    #[test]
    fn test_token_estimation() {
        let text = "Hello world, this is a test.";
        let tokens = compressor::tokenizer::estimate_tokens(text);
        assert!(tokens > 0);
        assert!(tokens < text.len());
    }
}

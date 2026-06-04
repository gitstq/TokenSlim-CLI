use super::{get_all_models, ModelInfo};

#[derive(Debug, Clone)]
pub struct CostComparison {
    pub model: ModelInfo,
    pub original_cost: f64,
    pub compressed_cost: f64,
    pub savings: f64,
    pub savings_percent: f64,
}

pub fn compare_costs(original_tokens: usize, compressed_tokens: usize, output_tokens: usize) -> Vec<CostComparison> {
    let models = get_all_models();
    let mut comparisons: Vec<CostComparison> = models
        .into_iter()
        .map(|model| {
            let original_cost = model.estimate_cost(original_tokens, output_tokens);
            let compressed_cost = model.estimate_cost(compressed_tokens, output_tokens);
            let savings = original_cost - compressed_cost;
            let savings_percent = if original_cost > 0.0 {
                (savings / original_cost) * 100.0
            } else {
                0.0
            };

            CostComparison {
                model,
                original_cost,
                compressed_cost,
                savings,
                savings_percent,
            }
        })
        .collect();

    // Sort by savings (descending)
    comparisons.sort_by(|a, b| b.savings.partial_cmp(&a.savings).unwrap());
    comparisons
}

pub fn format_usd(amount: f64) -> String {
    if amount < 0.0001 {
        format!("${:.6}", amount)
    } else if amount < 0.01 {
        format!("${:.4}", amount)
    } else if amount < 1.0 {
        format!("${:.2}", amount)
    } else {
        format!("${:.2}", amount)
    }
}

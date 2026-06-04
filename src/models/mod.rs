pub mod pricing;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelProvider {
    OpenAI,
    Anthropic,
    Google,
    DeepSeek,
    Zhipu,
    MiniMax,
    Moonshot,
    Baichuan,
    Qwen,
    Other,
}

impl std::fmt::Display for ModelProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelProvider::OpenAI => write!(f, "OpenAI"),
            ModelProvider::Anthropic => write!(f, "Anthropic"),
            ModelProvider::Google => write!(f, "Google"),
            ModelProvider::DeepSeek => write!(f, "DeepSeek"),
            ModelProvider::Zhipu => write!(f, "Zhipu AI"),
            ModelProvider::MiniMax => write!(f, "MiniMax"),
            ModelProvider::Moonshot => write!(f, "Moonshot"),
            ModelProvider::Baichuan => write!(f, "Baichuan"),
            ModelProvider::Qwen => write!(f, "Qwen"),
            ModelProvider::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub provider: ModelProvider,
    pub input_price_per_1m: f64,  // USD per 1M input tokens
    pub output_price_per_1m: f64, // USD per 1M output tokens
    pub context_window: usize,
    pub description: String,
}

impl ModelInfo {
    pub fn estimate_cost(&self, input_tokens: usize, output_tokens: usize) -> f64 {
        let input_cost = (input_tokens as f64 / 1_000_000.0) * self.input_price_per_1m;
        let output_cost = (output_tokens as f64 / 1_000_000.0) * self.output_price_per_1m;
        input_cost + output_cost
    }

    pub fn format_cost(&self, cost: f64) -> String {
        if cost < 0.0001 {
            format!("${:.6}", cost)
        } else if cost < 0.01 {
            format!("${:.4}", cost)
        } else {
            format!("${:.2}", cost)
        }
    }
}

pub fn get_all_models() -> Vec<ModelInfo> {
    vec![
        // OpenAI
        ModelInfo {
            id: "gpt-4o".to_string(),
            name: "GPT-4o".to_string(),
            provider: ModelProvider::OpenAI,
            input_price_per_1m: 2.50,
            output_price_per_1m: 10.00,
            context_window: 128000,
            description: "OpenAI's most capable multimodal model".to_string(),
        },
        ModelInfo {
            id: "gpt-4o-mini".to_string(),
            name: "GPT-4o Mini".to_string(),
            provider: ModelProvider::OpenAI,
            input_price_per_1m: 0.15,
            output_price_per_1m: 0.60,
            context_window: 128000,
            description: "Fast, affordable small model for focused tasks".to_string(),
        },
        ModelInfo {
            id: "o1".to_string(),
            name: "o1".to_string(),
            provider: ModelProvider::OpenAI,
            input_price_per_1m: 15.00,
            output_price_per_1m: 60.00,
            context_window: 200000,
            description: "Reasoning model for complex tasks".to_string(),
        },
        ModelInfo {
            id: "o3-mini".to_string(),
            name: "o3-mini".to_string(),
            provider: ModelProvider::OpenAI,
            input_price_per_1m: 1.10,
            output_price_per_1m: 4.40,
            context_window: 200000,
            description: "Fast reasoning model".to_string(),
        },
        // Anthropic
        ModelInfo {
            id: "claude-3-5-sonnet".to_string(),
            name: "Claude 3.5 Sonnet".to_string(),
            provider: ModelProvider::Anthropic,
            input_price_per_1m: 3.00,
            output_price_per_1m: 15.00,
            context_window: 200000,
            description: "Balanced intelligence and speed".to_string(),
        },
        ModelInfo {
            id: "claude-3-opus".to_string(),
            name: "Claude 3 Opus".to_string(),
            provider: ModelProvider::Anthropic,
            input_price_per_1m: 15.00,
            output_price_per_1m: 75.00,
            context_window: 200000,
            description: "Most powerful Claude model".to_string(),
        },
        ModelInfo {
            id: "claude-3-haiku".to_string(),
            name: "Claude 3 Haiku".to_string(),
            provider: ModelProvider::Anthropic,
            input_price_per_1m: 0.25,
            output_price_per_1m: 1.25,
            context_window: 200000,
            description: "Fastest Claude model".to_string(),
        },
        // Google
        ModelInfo {
            id: "gemini-1.5-pro".to_string(),
            name: "Gemini 1.5 Pro".to_string(),
            provider: ModelProvider::Google,
            input_price_per_1m: 3.50,
            output_price_per_1m: 10.50,
            context_window: 2000000,
            description: "Google's most capable model".to_string(),
        },
        ModelInfo {
            id: "gemini-1.5-flash".to_string(),
            name: "Gemini 1.5 Flash".to_string(),
            provider: ModelProvider::Google,
            input_price_per_1m: 0.35,
            output_price_per_1m: 1.05,
            context_window: 1000000,
            description: "Fast, cost-effective model".to_string(),
        },
        // DeepSeek
        ModelInfo {
            id: "deepseek-chat".to_string(),
            name: "DeepSeek-V3".to_string(),
            provider: ModelProvider::DeepSeek,
            input_price_per_1m: 0.27,
            output_price_per_1m: 1.10,
            context_window: 64000,
            description: "DeepSeek's general purpose model".to_string(),
        },
        ModelInfo {
            id: "deepseek-reasoner".to_string(),
            name: "DeepSeek-R1".to_string(),
            provider: ModelProvider::DeepSeek,
            input_price_per_1m: 0.55,
            output_price_per_1m: 2.19,
            context_window: 64000,
            description: "DeepSeek's reasoning model".to_string(),
        },
        // Zhipu (GLM)
        ModelInfo {
            id: "glm-4".to_string(),
            name: "GLM-4".to_string(),
            provider: ModelProvider::Zhipu,
            input_price_per_1m: 1.00,
            output_price_per_1m: 1.00,
            context_window: 128000,
            description: "Zhipu AI's flagship model".to_string(),
        },
        ModelInfo {
            id: "glm-4-flash".to_string(),
            name: "GLM-4-Flash".to_string(),
            provider: ModelProvider::Zhipu,
            input_price_per_1m: 0.10,
            output_price_per_1m: 0.10,
            context_window: 128000,
            description: "Zhipu AI's fast, affordable model".to_string(),
        },
        // MiniMax
        ModelInfo {
            id: "minimax-text-01".to_string(),
            name: "MiniMax-Text-01".to_string(),
            provider: ModelProvider::MiniMax,
            input_price_per_1m: 0.20,
            output_price_per_1m: 1.10,
            context_window: 4000000,
            description: "MiniMax's text model with 4M context".to_string(),
        },
        // Moonshot
        ModelInfo {
            id: "moonshot-v1-128k".to_string(),
            name: "Moonshot v1-128k".to_string(),
            provider: ModelProvider::Moonshot,
            input_price_per_1m: 0.60,
            output_price_per_1m: 0.60,
            context_window: 128000,
            description: "Moonshot AI's 128k context model".to_string(),
        },
        // Qwen
        ModelInfo {
            id: "qwen-max".to_string(),
            name: "Qwen-Max".to_string(),
            provider: ModelProvider::Qwen,
            input_price_per_1m: 0.50,
            output_price_per_1m: 1.00,
            context_window: 32000,
            description: "Alibaba's most capable Qwen model".to_string(),
        },
        ModelInfo {
            id: "qwen-plus".to_string(),
            name: "Qwen-Plus".to_string(),
            provider: ModelProvider::Qwen,
            input_price_per_1m: 0.20,
            output_price_per_1m: 0.60,
            context_window: 128000,
            description: "Balanced Qwen model".to_string(),
        },
    ]
}

pub fn get_model_by_id(id: &str) -> Option<ModelInfo> {
    get_all_models().into_iter().find(|m| m.id == id)
}

pub fn get_models_by_provider(provider: ModelProvider) -> Vec<ModelInfo> {
    get_all_models()
        .into_iter()
        .filter(|m| m.provider == provider)
        .collect()
}

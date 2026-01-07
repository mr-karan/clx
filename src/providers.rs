use crate::cli::ProviderType;

pub struct ProviderInfo {
    pub id: &'static str,
    pub provider_type: ProviderType,
    pub display_name: &'static str,
    pub default_model: &'static str,
    pub env_key: &'static str,
}

pub const ALL_PROVIDERS: &[ProviderInfo] = &[
    ProviderInfo {
        id: "openai",
        provider_type: ProviderType::Openai,
        display_name: "OpenAI",
        default_model: "gpt-4o-mini",
        env_key: "OPENAI_API_KEY",
    },
    ProviderInfo {
        id: "groq",
        provider_type: ProviderType::Groq,
        display_name: "Groq",
        default_model: "llama-3.3-70b-versatile",
        env_key: "GROQ_API_KEY",
    },
    ProviderInfo {
        id: "claude",
        provider_type: ProviderType::Claude,
        display_name: "Claude (Anthropic)",
        default_model: "claude-sonnet-4-20250514",
        env_key: "ANTHROPIC_API_KEY",
    },
    ProviderInfo {
        id: "ollama",
        provider_type: ProviderType::Ollama,
        display_name: "Ollama (local)",
        default_model: "llama3.2",
        env_key: "",
    },
    ProviderInfo {
        id: "openrouter",
        provider_type: ProviderType::Openrouter,
        display_name: "OpenRouter",
        default_model: "anthropic/claude-sonnet-4",
        env_key: "OPENROUTER_API_KEY",
    },
    ProviderInfo {
        id: "deepseek",
        provider_type: ProviderType::Deepseek,
        display_name: "DeepSeek",
        default_model: "deepseek-chat",
        env_key: "DEEPSEEK_API_KEY",
    },
    ProviderInfo {
        id: "gemini",
        provider_type: ProviderType::Gemini,
        display_name: "Gemini (Google)",
        default_model: "gemini-2.0-flash",
        env_key: "GEMINI_API_KEY",
    },
    ProviderInfo {
        id: "xai",
        provider_type: ProviderType::Xai,
        display_name: "xAI (Grok)",
        default_model: "grok-3-mini-fast",
        env_key: "XAI_API_KEY",
    },
];

impl ProviderInfo {
    pub fn for_provider(provider_type: ProviderType) -> &'static ProviderInfo {
        ALL_PROVIDERS
            .iter()
            .find(|p| p.provider_type == provider_type)
            .expect("Provider not found in ALL_PROVIDERS")
    }

    pub fn from_id(id: &str) -> Option<&'static ProviderInfo> {
        ALL_PROVIDERS.iter().find(|p| p.id == id)
    }
}

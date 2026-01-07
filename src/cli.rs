use clap::{Parser, Subcommand, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum ProviderType {
    Openai,
    Groq,
    Claude,
    Ollama,
    Openrouter,
    Deepseek,
    Gemini,
    Xai,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::Openai => write!(f, "openai"),
            ProviderType::Groq => write!(f, "groq"),
            ProviderType::Claude => write!(f, "claude"),
            ProviderType::Ollama => write!(f, "ollama"),
            ProviderType::Openrouter => write!(f, "openrouter"),
            ProviderType::Deepseek => write!(f, "deepseek"),
            ProviderType::Gemini => write!(f, "gemini"),
            ProviderType::Xai => write!(f, "xai"),
        }
    }
}

#[derive(Parser)]
#[command(name = "clx")]
#[command(about = "AI-powered CLI command generator")]
#[command(version)]
pub struct Cli {
    #[arg(short = 'p', long = "provider", value_enum)]
    pub provider: Option<ProviderType>,

    #[arg(short = 'm', long = "model")]
    pub model: Option<String>,

    #[arg(short = 't', long = "timeout", value_parser = parse_duration)]
    pub timeout: Option<u64>,

    #[arg(short = 'c', long = "config")]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(trailing_var_arg = true)]
    pub query: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Configure,
}

fn parse_duration(s: &str) -> Result<u64, String> {
    if let Some(stripped) = s.strip_suffix('s') {
        stripped
            .parse()
            .map_err(|_| format!("Invalid duration: {}", s))
    } else {
        s.parse()
            .map_err(|_| format!("Invalid duration: {}", s))
    }
}

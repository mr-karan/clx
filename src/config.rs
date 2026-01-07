use crate::cli::{Cli, ProviderType};
use crate::error::{ClxError, Result};
use crate::providers::ProviderInfo;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_provider")]
    pub provider: String,

    #[serde(default)]
    pub model: Option<String>,

    #[serde(default)]
    pub api_key: Option<String>,
}

fn default_provider() -> String {
    "openai".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            model: None,
            api_key: None,
        }
    }
}

impl Config {
    pub fn config_dir() -> Option<PathBuf> {
        dirs::config_dir().map(|c| c.join("clx"))
    }

    pub fn config_path() -> Option<PathBuf> {
        Self::config_dir().map(|c| c.join("config.json"))
    }

    pub fn load(custom_path: Option<&str>) -> Result<Self> {
        let path = custom_path.map(PathBuf::from).or_else(Self::config_path);

        match path {
            Some(p) if p.exists() => {
                let content = fs::read_to_string(&p)?;
                let config: Config = serde_json::from_str(&content)?;
                Ok(config)
            }
            _ => Ok(Config::default()),
        }
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir()
            .ok_or_else(|| ClxError::Config("Could not determine config directory".into()))?;

        fs::create_dir_all(&dir)?;

        let path = dir.join("config.json");
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn merge_with_cli(&mut self, cli: &Cli) {
        if let Some(provider) = &cli.provider {
            self.provider = provider.to_string();
        }

        if let Some(model) = &cli.model {
            self.model = Some(model.clone());
        }
    }

    pub fn provider_type(&self) -> Result<ProviderType> {
        match self.provider.as_str() {
            "openai" => Ok(ProviderType::Openai),
            "groq" => Ok(ProviderType::Groq),
            "claude" => Ok(ProviderType::Claude),
            "ollama" => Ok(ProviderType::Ollama),
            "openrouter" => Ok(ProviderType::Openrouter),
            "deepseek" => Ok(ProviderType::Deepseek),
            "gemini" => Ok(ProviderType::Gemini),
            "xai" => Ok(ProviderType::Xai),
            other => Err(ClxError::UnsupportedBackend(other.to_string())),
        }
    }

    pub fn effective_model(&self) -> String {
        self.model.clone().unwrap_or_else(|| {
            ProviderInfo::from_id(&self.provider)
                .map(|p| p.default_model.to_string())
                .unwrap_or_else(|| "gpt-4o-mini".to_string())
        })
    }
}

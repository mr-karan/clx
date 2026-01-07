use crate::config::Config;
use crate::error::{ClxError, Result};
use crate::providers::{ProviderInfo, ALL_PROVIDERS};
use inquire::{Select, Text};

struct ProviderChoice(&'static ProviderInfo);

impl std::fmt::Display for ProviderChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (default: {})", self.0.display_name, self.0.default_model)
    }
}

pub fn execute() -> Result<()> {
    println!("Configure clx\n");

    let options: Vec<ProviderChoice> = ALL_PROVIDERS.iter().map(ProviderChoice).collect();
    let selection = Select::new("Select your AI provider:", options)
        .with_help_message("Use arrow keys to navigate, Enter to select")
        .prompt()
        .map_err(|e| ClxError::Config(e.to_string()))?;

    let selected_provider = selection.0;

    let api_key = get_api_key(selected_provider)?;

    let model_input = Text::new("Model (leave empty for default):")
        .with_default(selected_provider.default_model)
        .prompt()
        .map_err(|e| ClxError::Config(e.to_string()))?;

    let model = if model_input == selected_provider.default_model {
        None
    } else {
        Some(model_input)
    };

    let config = Config {
        provider: selected_provider.id.to_string(),
        model,
        api_key,
    };

    config.save()?;

    println!("\nConfiguration saved to ~/.config/clx/config.json");

    Ok(())
}

fn get_api_key(provider: &ProviderInfo) -> Result<Option<String>> {
    if provider.env_key.is_empty() {
        println!("\n  Ollama runs locally - no API key needed.\n");
        return Ok(None);
    }

    let prompt = format!(
        "Enter your API key (or leave empty to use {}):",
        provider.env_key
    );

    let api_key = Text::new(&prompt)
        .prompt()
        .map_err(|e| ClxError::Config(e.to_string()))?;

    if api_key.is_empty() {
        Ok(None)
    } else {
        Ok(Some(api_key))
    }
}

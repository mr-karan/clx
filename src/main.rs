mod cli;
mod command;
mod config;
mod error;
mod prompt;
mod provider;
mod providers;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;
use error::{ClxError, Result};
use inquire::Text;
use provider::Provider;
use std::io::{self, IsTerminal, Read};
use std::process;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("\x1b[91merror:\x1b[0m {e}");
        process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();

    if let Some(Commands::Configure) = cli.command {
        return command::configure::execute();
    }

    let query = get_query(&cli)?;

    let mut config = Config::load(cli.config.as_deref())?;
    config.merge_with_cli(&cli);

    let provider_type = config.provider_type()?;
    let model = config.effective_model();

    let provider = Provider::new(provider_type, model, config.api_key)?;
    command::generate::execute(&provider, &query).await?;

    Ok(())
}

fn read_stdin() -> Option<String> {
    if io::stdin().is_terminal() {
        return None;
    }

    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_ok() {
        let input = input.trim().to_string();
        if !input.is_empty() {
            return Some(input);
        }
    }
    None
}

fn get_query(cli: &Cli) -> Result<String> {
    let args_query: String = cli.query.join(" ").trim().to_string();
    let stdin_query = read_stdin();

    match (args_query.is_empty(), stdin_query) {
        (false, Some(stdin)) => Ok(format!("{} {}", args_query, stdin)),
        (false, None) => Ok(args_query),
        (true, Some(stdin)) => Ok(stdin),
        (true, None) => prompt_user(),
    }
}

fn prompt_user() -> Result<String> {
    if !io::stdout().is_terminal() {
        return Err(ClxError::Config("No query provided".into()));
    }

    let input = Text::new("What command do you need?")
        .with_placeholder("e.g., show disk usage of current directory")
        .prompt()
        .map_err(|e| ClxError::Config(e.to_string()))?;

    let input = input.trim().to_string();
    if input.is_empty() {
        return Err(ClxError::Config("No query provided".into()));
    }

    Ok(input)
}

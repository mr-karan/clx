use crate::error::Result;
use crate::prompt::{CommandResult, Prompt};
use crate::provider::Provider;
use colored::Colorize;
use spinoff::{spinners, Color, Spinner};

pub async fn execute(provider: &Provider, query: &str) -> Result<()> {
    let mut spinner = Spinner::new(spinners::Dots, "Generating...".to_string(), Color::Blue);

    let prompt = Prompt::new(query);
    let response = provider.generate(prompt).await?;

    spinner.clear();

    match CommandResult::parse(&response) {
        Some(result) => print_result(&result),
        None => print_fallback(&response),
    }

    Ok(())
}

fn print_result(result: &CommandResult) {
    println!("{}", result.description.magenta().bold());

    for line in result.command.lines() {
        let line = line.trim();
        if !line.is_empty() {
            println!("{} {}", "$".green().bold(), line);
        }
    }

    if let Some(warning) = &result.warning {
        println!("\n{} {}", "Warning:".yellow().bold(), warning);
    }
}

fn print_fallback(response: &str) {
    let lines: Vec<&str> = response.lines().filter(|l| !l.is_empty()).collect();

    for (i, line) in lines.iter().enumerate() {
        if i == 0 {
            println!("{}", line.magenta().bold());
        } else {
            println!("{} {}", "$".green().bold(), line);
        }
    }
}

use crate::error::Result;
use crate::prompt::{CommandResult, Prompt};
use crate::provider::Provider;
use colored::Colorize;
use spinoff::{spinners, Color, Spinner};
use std::io::IsTerminal;

pub async fn execute(provider: &Provider, query: &str) -> Result<()> {
    let is_tty = std::io::stdout().is_terminal();
    let mut spinner = is_tty.then(|| {
        Spinner::new(spinners::Dots, "Generating...".to_string(), Color::Blue)
    });

    let prompt = Prompt::new(query);
    let response = provider.generate(prompt).await?;

    if let Some(ref mut s) = spinner {
        s.clear();
    }

    match CommandResult::parse(&response) {
        Some(result) => print_result(&result),
        None => print_fallback(&response),
    }

    Ok(())
}

fn print_result(result: &CommandResult) {
    let is_tty = std::io::stdout().is_terminal();
    let lines: Vec<&str> = result.command.lines().map(|l| l.trim()).filter(|l| !l.is_empty()).collect();

    if is_tty {
        for (i, line) in lines.iter().enumerate() {
            if i == lines.len() - 1 {
                println!("{} {}  {}", "$".green().bold(), line, format!("# {}", result.description).bright_black());
            } else {
                println!("{} {}", "$".green().bold(), line);
            }
        }
        if let Some(warning) = &result.warning {
            println!("{} {}", "Warning:".yellow().bold(), warning);
        }
    } else {
        println!("{}", result.description);
        for line in lines {
            println!("$ {}", line);
        }
        if let Some(warning) = &result.warning {
            println!("Warning: {}", warning);
        }
    }
}

fn print_fallback(response: &str) {
    let is_tty = std::io::stdout().is_terminal();
    let lines: Vec<&str> = response.lines().filter(|l| !l.is_empty()).collect();

    if is_tty {
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                println!("{}", line.magenta().bold());
            } else {
                println!("{} {}", "$".green().bold(), line);
            }
        }
    } else {
        for (i, line) in lines.iter().enumerate() {
            if i == 0 {
                println!("{}", line);
            } else {
                println!("$ {}", line);
            }
        }
    }
}

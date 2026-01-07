# AGENTS.md - Coding Agent Guidelines for clx

## Project Overview

clx is an AI-powered CLI command generator written in Rust. It takes natural language queries and generates shell commands using various AI providers (OpenAI, Groq, Claude, Ollama, OpenRouter, DeepSeek, Gemini, xAI).

## Build Commands

```bash
# Check compilation without building
cargo check

# Build debug binary
cargo build

# Build optimized release binary
cargo build --release

# Run directly
cargo run -- <query>
cargo run -- show disk usage

# Run with specific provider
cargo run -- -p groq "list files"

# Install locally
cargo install --path .
```

## Testing

```bash
# Run all tests
cargo test

# Run a single test by name
cargo test test_name

# Run tests in a specific module
cargo test module_name::

# Run tests with output
cargo test -- --nocapture

# Run ignored tests
cargo test -- --ignored
```

## Linting & Formatting

```bash
# Format code
cargo fmt

# Check formatting without changes
cargo fmt --check

# Run clippy linter
cargo clippy

# Clippy with warnings as errors (CI mode)
cargo clippy -- -D warnings
```

## Project Structure

```
src/
├── main.rs           # Entry point, CLI dispatch
├── cli.rs            # Clap CLI definitions, ProviderType enum
├── config.rs         # Config loading from ~/.config/clx/config.json
├── error.rs          # Error types using thiserror
├── prompt.rs         # System prompt construction
├── provider.rs       # AI provider abstraction (genai)
├── providers.rs      # Provider metadata (ALL_PROVIDERS)
└── command/
    ├── mod.rs
    ├── generate.rs   # Main command execution
    └── configure.rs  # Interactive setup
```

## Code Style Guidelines

### Imports

Order imports in groups separated by blank lines:
1. `crate::` imports (local modules)
2. External crate imports
3. `std::` imports

```rust
use crate::cli::ProviderType;
use crate::error::{ClxError, Result};

use genai::chat::{ChatMessage, ChatRequest};
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::PathBuf;
```

### Naming Conventions

- Types/Structs: `PascalCase` (e.g., `ProviderType`, `ClxError`)
- Functions/Methods: `snake_case` (e.g., `load_config`, `effective_model`)
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `ALL_PROVIDERS`, `DEFAULT_TIMEOUT`)
- Modules: `snake_case` (e.g., `providers.rs`, `command/generate.rs`)

### Error Handling

Use `thiserror` for error definitions:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClxError {
    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, ClxError>;
```

Propagate errors with `?` operator. Avoid `.unwrap()` and `.expect()` in library code.

### Async Code

Use `tokio` runtime. Entry point:

```rust
#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("\x1b[91merror:\x1b[0m {e}");
        std::process::exit(1);
    }
}
```

### CLI Definitions

Use clap derive macros:

```rust
use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "clx")]
#[command(about = "AI-powered CLI command generator")]
pub struct Cli {
    #[arg(short = 'p', long = "provider", value_enum)]
    pub provider: Option<ProviderType>,
}
```

### Configuration

- Config location: `~/.config/clx/config.json`
- Use `serde` for JSON serialization
- Use `dirs` crate for XDG paths
- Priority: CLI flags > config file > environment variables > defaults

### Adding a New Provider

1. Add variant to `ProviderType` enum in `cli.rs`
2. Add `ProviderInfo` entry to `ALL_PROVIDERS` in `providers.rs`
3. Add match arm in `provider.rs` for client construction
4. Add match arm in `config.rs` `provider_type()` method

### Dependencies

Key crates:
- `clap` - CLI parsing with derive macros
- `tokio` - Async runtime
- `genai` - Multi-provider AI client
- `serde` + `serde_json` - JSON serialization
- `thiserror` - Error definitions
- `dirs` - XDG config paths
- `spinoff` - Loading spinners
- `inquire` - Interactive prompts
- `colored` - Terminal colors

### Output Formatting

Use ANSI escape codes or `colored` crate:

```rust
use colored::Colorize;

println!("{}", description.magenta().bold());
println!("{} {}", "$".green().bold(), command);
```

### No Comments Policy

Code should be self-documenting. Avoid comments unless absolutely necessary for:
- Complex algorithms
- Security-related code
- Non-obvious performance optimizations
- Regex patterns

## Release Process

1. Update version in `Cargo.toml`
2. Commit changes
3. Tag release: `git tag v0.x.x`
4. Push tag: `git push origin v0.x.x`
5. GitHub Actions builds and releases binaries

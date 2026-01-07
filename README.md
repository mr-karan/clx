# clx

Turn natural language into ready-to-use terminal commands with AI.

## Examples

```sh
$ clx show disk usage of current directory
Showing disk usage of the current directory
$ du -sh .

$ clx kill process on port 8000
Kill process running on port 8000
$ lsof -t -i :8000 | xargs kill -9

$ clx find all rust files modified in the last week
Find Rust files modified in the last 7 days
$ find . -name "*.rs" -mtime -7
```

## Install

### Binary

Download the latest binary from [Releases](https://github.com/mr-karan/clx/releases).

### Cargo

```bash
cargo install clx
```

### Build from source

```bash
git clone https://github.com/mr-karan/clx
cd clx
cargo build --release
```

## Usage

```
clx [OPTIONS] [QUERY]...
clx configure
```

### Options

| Flag | Description |
|------|-------------|
| `-p, --provider <PROVIDER>` | AI provider (openai, groq, claude, ollama, openrouter, deepseek, gemini, xai) |
| `-m, --model <MODEL>` | Model to use (overrides config) |
| `-t, --timeout <TIMEOUT>` | Request timeout in seconds |
| `-c, --config <PATH>` | Path to config file |
| `-h, --help` | Show help |
| `-V, --version` | Show version |

### Input Methods

```sh
# Direct query
$ clx list all docker containers

# Interactive prompt (no arguments)
$ clx
? What command do you need? █

# Pipe from stdin
$ echo "compress all jpg files" | clx

# Combine args with stdin
$ echo "in the current directory" | clx find large files
```

## Configuration

### Interactive Setup

```bash
$ clx configure
```

This prompts you to select a provider, enter your API key, and choose a model.

### Manual Configuration

Create `~/.config/clx/config.json`:

```json
{
  "provider": "openai",
  "model": "gpt-4o-mini",
  "api_key": "sk-..."
}
```

All fields are optional. You can also use environment variables for API keys.

### Environment Variables

Set your API key as an environment variable instead of storing it in the config file:

```bash
export OPENAI_API_KEY="sk-..."
export GROQ_API_KEY="gsk_..."
```

## Providers

| Provider | Default Model | Environment Variable |
|----------|---------------|---------------------|
| openai | gpt-4o-mini | `OPENAI_API_KEY` |
| groq | llama-3.3-70b-versatile | `GROQ_API_KEY` |
| claude | claude-sonnet-4-20250514 | `ANTHROPIC_API_KEY` |
| ollama | llama3.2 | (none - runs locally) |
| openrouter | anthropic/claude-sonnet-4 | `OPENROUTER_API_KEY` |
| deepseek | deepseek-chat | `DEEPSEEK_API_KEY` |
| gemini | gemini-2.0-flash | `GEMINI_API_KEY` |
| xai | grok-3-mini-fast | `XAI_API_KEY` |

### Using a Different Provider

```sh
# One-off with a different provider
$ clx -p groq show memory usage

# Use a specific model
$ clx -p openai -m gpt-4o list running processes
```

## License

MIT

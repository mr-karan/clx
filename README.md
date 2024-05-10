# clx

clx is a useful utility that generates CLI commands using AI models for common operational tasks. It's inspired by [Shayan's clx](https://gist.github.com/Sh4yy/3941bf5014bc8c980fad797d85149b65) and enhanced to support configurable models and multiple backends including [Groq](https://groq.com/) and [OpenAI](https://openai.com/).

![](./docs/clx_v1.gif)

## Install

### Binary

Download the latest binary from [Releases](https://github.com/mr-karan/clx/releases).

### Go

```bash
go install github.com/mr-karan/clx@latest
```

## Example Usage

```sh
➜ clx show disk usage of current directory
Showing disk usage of the current directory
$ du -sh .
```

```sh
➜ clx kill process on port 8000
Kill process running on port 8000.
$ lsof -t -i :8000 | xargs kill -9
```

```sh
➜ clx backup a directory
Backup a directory to a tarball archive
$ tar -czf backup.tar.gz /path/to/directory
```

## Usage

```
➜ clx
NAME:
   clx - a CLI code generator

USAGE:
   clx [global options] command [command options]

COMMANDS:
   help, h  Shows a list of commands or help for one command

GLOBAL OPTIONS:
   --model value, -m value    Model to use for generating responses (default: "gpt-4-turbo")
   --backend value            Backend service to use (openai, groq) (default: "openai")
   --timeout value, -t value  Timeout for API requests (default: 30s)
   --config value, -c value   Path to config file
   --help, -h                 show help
```

## Configuration

`clx` can be configured using a config.toml file, which allows users to set default values for the backend service, model, and request timeout. Here’s an example of what the config.toml might look like:

```toml
backend = "groq" # Backend service (openai, groq)
model = "llama3-70b-8192" # Model to use
timeout = "30s" # Timeout for API requests
```

This file is read at runtime, and its settings are applied unless overridden by command-line flags. If the file does not exist, clx will create a default one using built-in settings. The file is located at `$HOME/clx.toml`.

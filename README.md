# clx

clx is a useful utility that generates CLI commands using AI models for common operational tasks. It's inspired by [Shayan's clx](https://gist.github.com/Sh4yy/3941bf5014bc8c980fad797d85149b65) and enhanced to support configurable models and multiple backends including [Groq](https://groq.com/) and [OpenAI](https://openai.com/).


![](./docs/clx.gif)

This project is inspired from [Shayan's clx](https://gist.github.com/Sh4yy/3941bf5014bc8c980fad797d85149b65) utility. I've ported it to add configurable models and different backends. Currently supports Groq and OpenAI.

You can use `--model` (`-m`) flag to specify the model name. Defaults to `llama3`.

## Install

### Binary

Download the latest binary from [Releases](https://github.com/mr-karan/clx/releases).

### Go

```bash
go install github.com/mr-karan/clx@latest
```

## Example Usage

```sh
❯ clx show disk usage of current directory
Displaying disk usage for the current directory
$ du -sh .
```

```sh
❯ clx use imagemagick to convert jpg to png and optimize for web usage
Converting JPEG images to PNG format with compression
$ convert -resize 50% input.jpg output.png && convert -trim -quality 75 -define jpeg:extent=1024 output.png output_optimized.png
```

```sh
➜ clx find out top 10 IPs in the nginx access log file
Viewing Top 10 Client IPs from Nginx Access Log
$ cat /var/log/nginx/access.log | awk '{print $1}' | sort | uniq -c | sort -rnk 1 | head -n 10
```

## Configuration

`clx` can be configured using a config.toml file, which allows users to set default values for the backend service, model, and request timeout. Here’s an example of what the config.toml might look like:

```toml
backend = "groq" # Backend service (openai, groq)
model = "llama3-70b-8192" # Model to use
timeout = "30s" # Timeout for API requests
```

This file is read at runtime, and its settings are applied unless overridden by command-line flags. If the file does not exist, clx will create a default one using built-in settings. The file is located at `$HOME/clx.toml`.

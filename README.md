# clx

_Generate CLI commands using AI for common ops_

![](./docs/clx.gif)

This project is an adaption of [Shayan's clx](https://gist.github.com/Sh4yy/3941bf5014bc8c980fad797d85149b65) tool. I've ported it to use locally available models through [ollama](https://ollama.com/library).

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
➜ clx show disk usage of current directory
Displaying disk usage for the current directory
du -sh .%
```

```sh
➜ clx use imagemagick to convert jpg to png and optimize for web usage
Converting JPEG images to PNG format with compression
convert -resize 50% input.jpg output.png && convert -trim -quality 75 -define jpeg:extent=1024 output.png output_optimized.png
```

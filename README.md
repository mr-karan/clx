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

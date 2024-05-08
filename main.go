package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"runtime"
	"strings"
	"time"

	"github.com/ollama/ollama/api"
	"github.com/urfave/cli/v2"
)

var (
	version = ""
	timeout = time.Second * 30
)

func getSystemPrompt(model string) string {
	systemInfo := fmt.Sprintf("OS: %s, Arch: %s", runtime.GOOS, runtime.GOARCH)
	prompt := fmt.Sprintf(`
		You are CLX, a CLI code generator. Respond with the CLI command to generate the code with only one short sentence description in first line.
		If the user asks for a specific language, respond with the CLI command to generate the code in that language.
		If CLI command is multiple lines, MUST separate each line with a newline character.
		Do not write any markdown. Do not write any code. No lengthy explanations either. Be concise and terse.
		System Info: %s
		Model: %s

		First line is the description in one sentence.
		Example output:

		Building and installing a Go binary
		go build main.go
		go install main
	`, systemInfo, model)

	return prompt
}

func askAI(phrase string, model string) error {
	client, err := api.ClientFromEnvironment()
	if err != nil {
		log.Fatal(err)
	}

	messages := []api.Message{
		{Role: "system", Content: getSystemPrompt(model)},
		{Role: "user", Content: phrase},
	}

	stream := false

	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	req := &api.ChatRequest{
		Model:    model,
		Messages: messages,
		Stream:   &stream,
		Options: map[string]interface{}{
			"seed":        42,
			"temperature": 0.2,
			"top_k":       10,
			"top_p":       0.5,
		},
	}

	if err := client.Chat(ctx, req, func(resp api.ChatResponse) error {
		content := resp.Message.Content
		lines := strings.Split(content, "\n")
		firstLine := true

		for _, line := range lines {
			if line != "" {
				if firstLine {
					fmt.Printf("\x1b[1;35m%s\x1b[0m", line) // Print first line in purple
					firstLine = false
				} else {
					fmt.Printf("\n\x1b[1;32m$ \x1b[0m%s", line) // Print subsequent lines with green prompt
				}
			}
		}

		if len(lines) > 1 {
			fmt.Println() // Ensure ending on a new line
		}

		return nil
	}); err != nil {
		return err
	}

	return nil
}

func main() {
	app := &cli.App{
		Name:    "clx",
		Usage:   "a CLI code generator",
		Version: version,
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    "model",
				Value:   "llama3",
				Usage:   "Model to use for generating responses",
				Aliases: []string{"m"},
			},
		},
		Action: func(c *cli.Context) error {
			if c.Bool("version") {
				fmt.Println(version)
				return nil
			}
			phrase := strings.Join(c.Args().Slice(), " ") // Join all arguments into a single string
			if phrase == "" {
				cli.ShowAppHelpAndExit(c, 1)
			}
			model := c.String("model")
			return askAI(phrase, model)
		},
	}
	err := app.Run(os.Args)
	if err != nil {
		log.Fatal(err)
	}
}

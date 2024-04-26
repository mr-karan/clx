package main

import (
	"context"
	"fmt"
	"log"
	"os"
	"runtime"
	"strings"

	"github.com/ollama/ollama/api"
)

func getSystemPrompt() string {
	systemInfo := fmt.Sprintf("OS: %s, Arch: %s", runtime.GOOS, runtime.GOARCH)
	prompt := fmt.Sprintf(`
		You are CLX, a CLI code generator. Respond with the CLI command to generate the code with only one short sentence description in first line.
		If the user asks for a specific language, respond with the CLI command to generate the code in that language.
		If CLI command is multiple lines, separate each line with a newline character.
		Do not write any markdown. Do not write any code.
		System Info: %s

		First line is the description in one sentence.
		Example output:

		Building and installing a Go binary
		go build main.go
		go install main
	`, systemInfo)

	return prompt
}

func askAI(phrase string) error {
	client, err := api.ClientFromEnvironment()
	if err != nil {
		log.Fatal(err)
	}

	messages := []api.Message{
		api.Message{
			Role:    "system",
			Content: getSystemPrompt(),
		},
		api.Message{
			Role:    "user",
			Content: phrase,
		},
	}

	ctx := context.Background()
	req := &api.ChatRequest{
		Model:    "llama3",
		Messages: messages,
	}

	err = client.Chat(ctx, req, func(resp api.ChatResponse) error {
		content := resp.Message.Content
		lines := strings.Split(content, "\n")
		firstLine := true

		for i, line := range lines {
			if line != "" {
				if firstLine {
					fmt.Printf("\x1b[1;35m%s\x1b[0m", line)
				} else {
					fmt.Print(line)
				}
			}

			if !firstLine && len(lines) > 1 && i != 0 {
				fmt.Print("\n\x1b[1;32m$ \x1b[0m")
			}

			if firstLine && len(lines) > 1 {
				fmt.Print("\n")
				firstLine = false
			}
		}

		return nil
	})
	if err != nil {
		return err
	}
	return nil
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: clx <prompt>")
		os.Exit(1)
	}
	phrase := strings.Join(os.Args[1:], " ")
	if err := askAI(phrase); err != nil {
		log.Fatal(err)
	}
}

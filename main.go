package main

import (
	"context"
	"errors"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"time"

	"github.com/BurntSushi/toml"
	"github.com/tmc/langchaingo/llms"
	"github.com/tmc/langchaingo/llms/openai"
	"github.com/urfave/cli/v2"
)

const (
	DefaultModel   = "gpt-4-turbo"
	DefaultBackend = "openai"
	DefaultTimeout = "30s" // String format for duration, used in TOML and time.ParseDuration
)

var (
	version = ""
)

// Config structure to hold the configuration from TOML file.
type Config struct {
	Backend string `toml:"backend"`
	Model   string `toml:"model"`
	Timeout string `toml:"timeout"`
}

func getSystemPrompt() string {
	systemInfo := fmt.Sprintf("OS: %s, Arch: %s", runtime.GOOS, runtime.GOARCH)
	return fmt.Sprintf(`
		You are CLX, a CLI code generator. Respond with the CLI command to generate the code with only one short sentence description in first line.
		If the user asks for a specific language, respond with the CLI command to generate the code in that language.
		If CLI command is multiple lines, MUST separate each line with a newline character.
		Do not write any markdown. Do not write any code. No lengthy explanations either. Be concise and terse.
		System Info: %s

		First line is the description in one sentence.
		Example output:

		Building and installing a Go binary
		go build main.go
		go install main
	`, systemInfo)
}

func loadConfig(configPath string) (*Config, error) {
	if configPath == "" {
		dirname, err := os.UserHomeDir()
		if err != nil {
			return nil, err
		}
		configPath = filepath.Join(dirname, "clx.toml")
	}

	var config Config
	_, err := toml.DecodeFile(configPath, &config)
	if err != nil {
		if errors.Is(err, os.ErrNotExist) {
			// Create the default config file if it does not exist
			config = Config{
				Backend: DefaultBackend,
				Model:   DefaultModel,
				Timeout: DefaultTimeout,
			}
			file, err := os.Create(configPath)
			if err != nil {
				return nil, fmt.Errorf("failed to create default config file: %w", err)
			}
			defer file.Close()
			encoder := toml.NewEncoder(file)
			if err := encoder.Encode(config); err != nil {
				return nil, fmt.Errorf("failed to write default config values: %w", err)
			}
			return &config, nil
		}
		return nil, err
	}
	return &config, nil
}

func askAI(phrase string, model string, backend string, timeout time.Duration) error {
	var llm llms.Model
	var err error
	switch backend {
	case "openai":
		llm, err = openai.New(openai.WithModel(model))
	case "groq":
		llm, err = openai.New(
			openai.WithModel(model),
			openai.WithBaseURL("https://api.groq.com/openai/v1"),
		)
	default:
		return fmt.Errorf("unsupported backend: %s", backend)
	}
	if err != nil || llm == nil {
		log.Fatal(fmt.Errorf("failed to initialize LLM client: %w", err))
	}

	ctx, cancel := context.WithTimeout(context.Background(), timeout)
	defer cancel()

	content := []llms.MessageContent{
		llms.TextParts(llms.ChatMessageTypeSystem, getSystemPrompt()),
		llms.TextParts(llms.ChatMessageTypeHuman, phrase),
	}

	// Capture the output and process for coloring
	var outputBuilder strings.Builder
	_, err = llm.GenerateContent(ctx, content,
		llms.WithTemperature(0.2),
		llms.WithMaxTokens(4096),
		llms.WithTopP(0.5),
		llms.WithTopK(10),
		llms.WithStreamingFunc(func(ctx context.Context, chunk []byte) error {
			outputBuilder.Write(chunk)
			return nil
		}),
	)
	if err != nil {
		return err
	}

	// Process the full output for coloring and formatting
	output := outputBuilder.String()
	lines := strings.Split(output, "\n")
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
}

func main() {
	app := &cli.App{
		Name:    "clx",
		Usage:   "a CLI code generator",
		Version: version,
		Flags: []cli.Flag{
			&cli.StringFlag{
				Name:    "model",
				Value:   "gpt-4-turbo",
				Usage:   "Model to use for generating responses",
				Aliases: []string{"m"},
			},
			&cli.StringFlag{
				Name:  "backend",
				Value: "openai",
				Usage: "Backend service to use (openai, groq)",
			},
			&cli.DurationFlag{
				Name:    "timeout",
				Value:   30 * time.Second,
				Usage:   "Timeout for API requests",
				Aliases: []string{"t"},
			},
			&cli.StringFlag{
				Name:    "config",
				Usage:   "Path to config file",
				Value:   "",
				Aliases: []string{"c"},
			},
		},
		Before: func(c *cli.Context) error {
			configPath := c.String("config")
			config, err := loadConfig(configPath)
			if err != nil {
				fmt.Println("Warning: Failed to load config file, using default settings.")
				return nil // Continue even if config fails to load
			}

			// Apply configuration values if not overridden by command-line flags
			if !c.IsSet("model") && config.Model != "" {
				c.Set("model", config.Model)
			}
			if !c.IsSet("backend") && config.Backend != "" {
				c.Set("backend", config.Backend)
			}
			if !c.IsSet("timeout") && config.Timeout != "" {
				duration, err := time.ParseDuration(config.Timeout)
				if err == nil {
					c.Set("timeout", duration.String())
				}
			}
			return nil
		},
		Action: func(c *cli.Context) error {
			if c.Bool("version") {
				fmt.Println(version)
				return nil
			}
			// Join all arguments into a single string/
			phrase := strings.Join(c.Args().Slice(), " ")
			if phrase == "" {
				cli.ShowAppHelpAndExit(c, 1)
			}
			model := c.String("model")
			backend := c.String("backend")
			timeout := c.Duration("timeout")
			if backend != "openai" && backend != "groq" {
				return fmt.Errorf("unsupported backend: %s", backend)
			}
			return askAI(phrase, model, backend, timeout)
		},
	}
	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}

# clx.wezterm

WezTerm plugin for [clx](https://github.com/mr-karan/clx) - AI-powered CLI command generator.

Press a keybinding, describe what you need, and get the command inserted into your terminal.

## Requirements

- [WezTerm](https://wezfurlong.org/wezterm/) terminal
- [clx](https://github.com/mr-karan/clx) installed and configured

## Installation

Add to your `wezterm.lua`:

```lua
local wezterm = require("wezterm")
local config = wezterm.config_builder()

local clx = wezterm.plugin.require("https://github.com/mr-karan/clx/wezterm-plugin")
clx.apply_to_config(config)

return config
```

Or clone locally and use:

```lua
local clx = require("path.to.clx-wezterm.plugin")
clx.apply_to_config(config)
```

## Usage

| Keybinding | Description |
|------------|-------------|
| `Cmd+I` / `Super+I` | Open prompt and generate command |
| `Cmd+Shift+I` / `Super+Shift+I` | Open prompt with terminal context (shares recent output with AI) |

### Example

1. Press `Cmd+I`
2. Type: "find large files over 100mb"
3. See the description and command appear in your terminal

```
Find files larger than 100MB in current directory
$ find . -type f -size +100M
```

The command is inserted but not executed, so you can review before pressing Enter.

## Configuration

```lua
clx.apply_to_config(config, {
    -- Path to clx binary (default: "clx")
    clx_path = "/usr/local/bin/clx",

    -- Primary keybinding (default: Cmd+I)
    keybinding = {
        key = "i",
        mods = "SUPER",
    },

    -- Keybinding with terminal context (default: Cmd+Shift+I)
    keybinding_with_pane = {
        key = "I",
        mods = "SUPER",
    },

    -- Show loading indicator (default: true)
    show_loading = true,

    -- Number of terminal lines to share as context (default: 150)
    share_n_lines = 150,
})
```

### Using with Different Modifiers

On Linux, you might prefer `Ctrl` instead of `Super`:

```lua
clx.apply_to_config(config, {
    keybinding = {
        key = "i",
        mods = "CTRL|ALT",
    },
    keybinding_with_pane = {
        key = "I",
        mods = "CTRL|ALT|SHIFT",
    },
})
```

## How It Works

1. Plugin opens WezTerm's input prompt
2. Your query is passed to the `clx` binary
3. `clx` calls your configured AI provider (OpenAI, Groq, Claude, etc.)
4. The generated command is inserted into your terminal pane

Make sure `clx` is configured with your preferred AI provider:

```bash
clx configure
```

## License

MIT

local wezterm = require("wezterm")
local act = wezterm.action

local default_config = {
    clx_path = "clx",
    keybinding = {
        key = "i",
        mods = "SUPER",
    },
    keybinding_with_pane = {
        key = "I",
        mods = "SUPER",
    },
    timeout = 30,
    show_loading = true,
    share_n_lines = 150,
}

local function merge_config(user_config)
    local config = {}
    for k, v in pairs(default_config) do
        config[k] = v
    end
    if user_config then
        for k, v in pairs(user_config) do
            config[k] = v
        end
    end
    return config
end

local function parse_clx_output(output)
    if not output or output == "" then
        return nil, nil
    end

    local lines = {}
    for line in output:gmatch("[^\r\n]+") do
        table.insert(lines, line)
    end

    if #lines == 0 then
        return nil, nil
    end

    local description = lines[1]
    local commands = {}

    for i = 2, #lines do
        local line = lines[i]
        local cmd = line:match("^%$%s*(.+)$")
        if cmd then
            table.insert(commands, cmd)
        end
    end

    local command = table.concat(commands, " && ")
    return description, command
end

local function show_loading(pane, show)
    if show then
        pane:inject_output("\r\n\x1b[34m...\x1b[0m clx is thinking...")
    end
end

local function handle_clx_request(window, pane, prompt, config)
    if not prompt or prompt:match("^%s*$") then
        wezterm.log_info("clx.wezterm: Empty prompt, cancelling")
        return
    end

    local full_prompt = prompt

    if config._share_pane_history then
        local history
        if config.share_n_lines ~= nil then
            history = pane:get_logical_lines_as_text(config.share_n_lines)
        else
            history = pane:get_logical_lines_as_text()
        end
        full_prompt = prompt .. "\n\nTerminal context:\n" .. history
    end

    if config.show_loading then
        show_loading(pane, true)
    end

    local cmd = string.format(
        "%s %s 2>&1",
        config.clx_path,
        wezterm.shell_quote_arg(full_prompt)
    )

    local success, stdout, stderr = wezterm.run_child_process({ "sh", "-c", cmd })

    pane:send_text("\x15")
    pane:send_text("\r")

    if success then
        local output = stdout or ""
        wezterm.log_info("clx.wezterm: Response received")

        local description, command = parse_clx_output(output)

        if description then
            pane:inject_output("\r\n\x1b[35m" .. description .. "\x1b[0m")
        end

        if command and command ~= "" then
            pane:send_text(command)
        else
            pane:inject_output("\r\n\x1b[33mNo command generated\x1b[0m")
        end
    else
        local error_msg = stderr or "Unknown error"
        wezterm.log_error("clx.wezterm: Error - " .. error_msg)
        pane:inject_output("\r\n\x1b[31mclx error: " .. error_msg .. "\x1b[0m")
    end
end

local function apply_to_config(wezterm_config, user_config)
    local config = merge_config(user_config)

    if wezterm_config.keys == nil then
        wezterm_config.keys = {}
    end

    table.insert(wezterm_config.keys, {
        key = config.keybinding.key,
        mods = config.keybinding.mods,
        action = act.PromptInputLine({
            description = "clx: What command do you need?",
            action = wezterm.action_callback(function(window, pane, line)
                if line then
                    handle_clx_request(window, pane, line, config)
                else
                    wezterm.log_info("clx.wezterm: Request cancelled by user")
                end
            end),
        }),
    })

    table.insert(wezterm_config.keys, {
        key = config.keybinding_with_pane.key,
        mods = config.keybinding_with_pane.mods,
        action = act.PromptInputLine({
            description = "clx: What command do you need? (sharing terminal context)",
            action = wezterm.action_callback(function(window, pane, line)
                config._share_pane_history = true
                if line then
                    handle_clx_request(window, pane, line, config)
                else
                    wezterm.log_info("clx.wezterm: Request cancelled by user")
                end
                config._share_pane_history = false
            end),
        }),
    })

    wezterm.log_info("clx.wezterm: Plugin loaded (clx_path=" .. config.clx_path .. ")")
end

return {
    apply_to_config = apply_to_config,
}

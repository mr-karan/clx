use std::env::consts::{ARCH, OS};

pub struct Prompt {
    pub system: String,
    pub user: String,
}

impl Prompt {
    pub fn new(query: &str) -> Self {
        let shell = std::env::var("SHELL")
            .unwrap_or_else(|_| "bash".to_string())
            .split('/')
            .last()
            .unwrap_or("bash")
            .to_string();

        let system = format!(
            r#"You are a CLI command generator. Generate shell commands for the user's request.

System: {os} {arch}
Shell: {shell}

Respond ONLY in this exact XML format:
<description>One sentence describing what the command does</description>
<command>The shell command(s)</command>
<warning>Only include for dangerous commands like rm -rf, dd, etc. Otherwise omit this tag entirely.</warning>

Rules:
- Commands must be valid for {os} with {shell}
- For multiple commands, separate with && or newlines inside the command tag
- No markdown, no backticks, no extra text outside the XML tags
- Keep description under 80 characters
- Be precise and avoid unnecessary flags"#,
            os = OS,
            arch = ARCH,
            shell = shell
        );

        Self {
            system,
            user: query.to_string(),
        }
    }
}

pub struct CommandResult {
    pub description: String,
    pub command: String,
    pub warning: Option<String>,
}

impl CommandResult {
    pub fn parse(response: &str) -> Option<Self> {
        let description = extract_tag(response, "description")?;
        let command = extract_tag(response, "command")?;
        let warning = extract_tag(response, "warning");

        Some(Self {
            description,
            command,
            warning,
        })
    }
}

fn extract_tag(text: &str, tag: &str) -> Option<String> {
    let start_tag = format!("<{}>", tag);
    let end_tag = format!("</{}>", tag);

    let start = text.find(&start_tag)? + start_tag.len();
    let end = text.find(&end_tag)?;

    if start >= end {
        return None;
    }

    Some(text[start..end].trim().to_string())
}

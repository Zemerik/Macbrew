use serde::{Serialize, Deserialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct TerminalConfig {
    pub prompt_style: PromptStyle,
    pub history_size: usize,
    pub auto_completion: bool,
    pub syntax_highlighting: bool,
    pub colors: ColorScheme,
    pub aliases: std::collections::HashMap<String, String>,
    pub environment: std::collections::HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptStyle {
    pub show_hostname: bool,
    pub show_username: bool,
    pub show_path: bool,
    pub show_git_branch: bool,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorScheme {
    pub prompt: String,
    pub command: String,
    pub output: String,
    pub error: String,
    pub success: String,
    pub warning: String,
}

impl Default for TerminalConfig {
    fn default() -> Self {
        TerminalConfig {
            prompt_style: PromptStyle {
                show_hostname: true,
                show_username: true,
                show_path: true,
                show_git_branch: false,
                format: "{username}@{hostname}:{path} $ ".to_string(),
            },
            history_size: 1000,
            auto_completion: true,
            syntax_highlighting: true,
            colors: ColorScheme {
                prompt: "\x1b[32m".to_string(),      // Green
                command: "\x1b[36m".to_string(),     // Cyan
                output: "\x1b[0m".to_string(),       // Reset
                error: "\x1b[31m".to_string(),       // Red
                success: "\x1b[32m".to_string(),     // Green
                warning: "\x1b[33m".to_string(),     // Yellow
            },
            aliases: Self::default_aliases(),
            environment: Self::default_environment(),
        }
    }
}

impl TerminalConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path();
        
        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let config: TerminalConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = TerminalConfig::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path();
        
        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, json)?;
        Ok(())
    }

    pub fn get_alias(&self, command: &str) -> Option<&String> {
        self.aliases.get(command)
    }

    pub fn set_alias(&mut self, name: String, value: String) {
        self.aliases.insert(name, value);
    }

    pub fn remove_alias(&mut self, name: &str) -> Option<String> {
        self.aliases.remove(name)
    }

    pub fn list_aliases(&self) -> &std::collections::HashMap<String, String> {
        &self.aliases
    }

    pub fn get_env_var(&self, name: &str) -> Option<&String> {
        self.environment.get(name)
    }

    pub fn set_env_var(&mut self, name: String, value: String) {
        self.environment.insert(name, value);
    }

    pub fn unset_env_var(&mut self, name: &str) -> Option<String> {
        self.environment.remove(name)
    }

    pub fn list_env_vars(&self) -> &std::collections::HashMap<String, String> {
        &self.environment
    }

    pub fn update_prompt_style(&mut self, style: PromptStyle) {
        self.prompt_style = style;
    }

    pub fn update_color_scheme(&mut self, colors: ColorScheme) {
        self.colors = colors;
    }

    pub fn set_history_size(&mut self, size: usize) {
        self.history_size = size;
    }

    pub fn toggle_auto_completion(&mut self) {
        self.auto_completion = !self.auto_completion;
    }

    pub fn toggle_syntax_highlighting(&mut self) {
        self.syntax_highlighting = !self.syntax_highlighting;
    }

    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("terminal-emulator");
        path.push("config.json");
        path
    }

    fn default_aliases() -> std::collections::HashMap<String, String> {
        let mut aliases = std::collections::HashMap::new();
        aliases.insert("ll".to_string(), "ls -la".to_string());
        aliases.insert("la".to_string(), "ls -A".to_string());
        aliases.insert("l".to_string(), "ls -CF".to_string());
        aliases.insert("..".to_string(), "cd ..".to_string());
        aliases.insert("...".to_string(), "cd ../..".to_string());
        aliases.insert("g".to_string(), "git".to_string());
        aliases.insert("gs".to_string(), "git status".to_string());
        aliases.insert("ga".to_string(), "git add".to_string());
        aliases.insert("gc".to_string(), "git commit".to_string());
        aliases.insert("gp".to_string(), "git push".to_string());
        aliases.insert("gl".to_string(), "git pull".to_string());
        aliases.insert("b".to_string(), "brew".to_string());
        aliases.insert("bi".to_string(), "brew install".to_string());
        aliases.insert("bu".to_string(), "brew update".to_string());
        aliases.insert("bg".to_string(), "brew upgrade".to_string());
        aliases.insert("p".to_string(), "python".to_string());
        aliases.insert("py".to_string(), "python".to_string());
        aliases
    }

    fn default_environment() -> std::collections::HashMap<String, String> {
        let mut env = std::collections::HashMap::new();
        env.insert("TERM".to_string(), "xterm-256color".to_string());
        env.insert("SHELL".to_string(), "/bin/zsh".to_string());
        env.insert("EDITOR".to_string(), "nano".to_string());
        env.insert("PAGER".to_string(), "less".to_string());
        env.insert("LANG".to_string(), "en_US.UTF-8".to_string());
        env.insert("LC_ALL".to_string(), "en_US.UTF-8".to_string());
        env.insert("PATH".to_string(), "/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin".to_string());
        env
    }
}

impl PromptStyle {
    pub fn new() -> Self {
        Self {
            show_hostname: true,
            show_username: true,
            show_path: true,
            show_git_branch: false,
            format: "{username}@{hostname}:{path} $ ".to_string(),
        }
    }

    pub fn format_prompt(&self, username: &str, hostname: &str, path: &str, git_branch: Option<&str>) -> String {
        let mut formatted = self.format.clone();
        
        formatted = formatted.replace("{username}", username);
        formatted = formatted.replace("{hostname}", hostname);
        formatted = formatted.replace("{path}", path);
        
        if let Some(branch) = git_branch {
            formatted = formatted.replace("{git_branch}", &format!("({})", branch));
        } else {
            formatted = formatted.replace("{git_branch}", "");
        }
        
        formatted
    }
}

impl ColorScheme {
    pub fn new() -> Self {
        Self {
            prompt: "\x1b[32m".to_string(),
            command: "\x1b[36m".to_string(),
            output: "\x1b[0m".to_string(),
            error: "\x1b[31m".to_string(),
            success: "\x1b[32m".to_string(),
            warning: "\x1b[33m".to_string(),
        }
    }

    pub fn apply_prompt(&self, text: &str) -> String {
        format!("{}{}{}", self.prompt, text, self.output)
    }

    pub fn apply_command(&self, text: &str) -> String {
        format!("{}{}{}", self.command, text, self.output)
    }

    pub fn apply_error(&self, text: &str) -> String {
        format!("{}{}{}", self.error, text, self.output)
    }

    pub fn apply_success(&self, text: &str) -> String {
        format!("{}{}{}", self.success, text, self.output)
    }

    pub fn apply_warning(&self, text: &str) -> String {
        format!("{}{}{}", self.warning, text, self.output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TerminalConfig::default();
        assert_eq!(config.history_size, 1000);
        assert!(config.auto_completion);
        assert!(config.syntax_highlighting);
    }

    #[test]
    fn test_prompt_style_formatting() {
        let style = PromptStyle::new();
        let formatted = style.format_prompt("user", "host", "/path", Some("main"));
        assert!(formatted.contains("user"));
        assert!(formatted.contains("host"));
        assert!(formatted.contains("/path"));
        assert!(formatted.contains("(main)"));
    }

    #[test]
    fn test_color_scheme() {
        let colors = ColorScheme::new();
        let colored = colors.apply_error("Error message");
        assert!(colored.contains("\x1b[31m")); // Red color code
        assert!(colored.contains("\x1b[0m"));  // Reset color code
    }

    #[test]
    fn test_aliases() {
        let config = TerminalConfig::default();
        assert_eq!(config.get_alias("ll"), Some(&"ls -la".to_string()));
        assert_eq!(config.get_alias("nonexistent"), None);
    }
} 
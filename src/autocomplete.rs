use std::collections::HashMap;
use std::path::Path;
use glob::glob;
use rustyline::completion::{Completer, FilenameCompleter};
use rustyline::hint::Hinter;
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::validate::{Validator, ValidationContext, ValidationResult};
use rustyline::{Context, Result as LineResult};

pub struct TerminalCompleter {
    commands: Vec<String>,
    aliases: HashMap<String, String>,
    filename_completer: FilenameCompleter,
    bracket_highlighter: MatchingBracketHighlighter,
}

impl TerminalCompleter {
    pub fn new() -> Self {
        let mut commands = vec![
            "ls", "cd", "pwd", "cat", "echo", "mkdir", "rm", "cp", "mv",
            "grep", "find", "chmod", "chown", "ps", "top", "kill",
            "python", "brew", "git", "npm", "yarn", "docker",
            "ssh", "scp", "curl", "wget", "ping", "netstat",
            "vim", "nano", "less", "more", "head", "tail",
            "sort", "uniq", "cut", "paste", "tr", "sed", "awk",
            "wc", "file", "stat", "du", "df", "tar", "gzip",
            "gunzip", "bzip2", "bunzip2", "zip", "unzip",
            "history", "help", "exit", "quit", "clear", "date",
            "whoami", "sudo", "su", "useradd", "userdel", "passwd",
            "systemctl", "service", "cron", "at", "crontab",
            "screen", "tmux", "nohup", "bg", "fg", "jobs"
        ].into_iter().map(String::from).collect();

        let mut aliases = HashMap::new();
        aliases.insert("ll".to_string(), "ls -la".to_string());
        aliases.insert("la".to_string(), "ls -a".to_string());
        aliases.insert("l".to_string(), "ls -CF".to_string());
        aliases.insert("g".to_string(), "git".to_string());
        aliases.insert("p".to_string(), "python".to_string());
        aliases.insert("n".to_string(), "npm".to_string());
        aliases.insert("y".to_string(), "yarn".to_string());
        aliases.insert("d".to_string(), "docker".to_string());

        TerminalCompleter {
            commands,
            aliases,
            filename_completer: FilenameCompleter::new(),
            bracket_highlighter: MatchingBracketHighlighter::new(),
        }
    }

    pub fn complete_command(&self, input: &str) -> Vec<String> {
        let mut completions = Vec::new();
        let input_lower = input.to_lowercase();

        // Complete commands
        for cmd in &self.commands {
            if cmd.to_lowercase().starts_with(&input_lower) {
                completions.push(cmd.clone());
            }
        }

        // Complete aliases
        for (alias, _) in &self.aliases {
            if alias.to_lowercase().starts_with(&input_lower) {
                completions.push(alias.clone());
            }
        }

        completions.sort();
        completions.dedup();
        completions
    }

    pub fn complete_file(&self, input: &str) -> Vec<String> {
        let mut completions = Vec::new();
        
        if let Some(expanded) = shellexpand::full(input).ok() {
            let path = Path::new(&*expanded);
            
            if let Some(parent) = path.parent() {
                if let Some(file_name) = path.file_name() {
                    let pattern = if let Some(name) = file_name.to_str() {
                        format!("{}/*{}*", parent.display(), name)
                    } else {
                        return completions;
                    };

                    if let Ok(entries) = glob(&pattern) {
                        for entry in entries {
                            if let Ok(path) = entry {
                                if let Some(name) = path.file_name() {
                                    if let Some(name_str) = name.to_str() {
                                        completions.push(name_str.to_string());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        completions.sort();
        completions.dedup();
        completions
    }

    pub fn complete_path(&self, input: &str) -> Vec<String> {
        let mut completions = Vec::new();
        
        if let Some(expanded) = shellexpand::full(input).ok() {
            let path = Path::new(&*expanded);
            
            if let Some(parent) = path.parent() {
                if let Some(file_name) = path.file_name() {
                    let pattern = if let Some(name) = file_name.to_str() {
                        format!("{}/*{}*", parent.display(), name)
                    } else {
                        return completions;
                    };

                    if let Ok(entries) = glob(&pattern) {
                        for entry in entries {
                            if let Ok(path) = entry {
                                if let Some(name) = path.file_name() {
                                    if let Some(name_str) = name.to_str() {
                                        let mut completion = name_str.to_string();
                                        if path.is_dir() {
                                            completion.push('/');
                                        }
                                        completions.push(completion);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        completions.sort();
        completions.dedup();
        completions
    }

    pub fn get_suggestions(&self, input: &str) -> Vec<String> {
        let words: Vec<&str> = input.split_whitespace().collect();
        if words.is_empty() {
            return self.commands.clone();
        }

        let first_word = words[0];
        if words.len() == 1 {
            // Complete first word (command)
            self.complete_command(first_word)
        } else {
            // Complete subsequent words (arguments)
            let last_word = words.last().unwrap();
            self.complete_path(last_word)
        }
    }
}

impl Completer for TerminalCompleter {
    type Candidate = String;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> LineResult<(usize, Vec<Self::Candidate>)> {
        let (start, candidates) = if pos == 0 {
            (0, self.complete_command(""))
        } else {
            let word_start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
            let word = &line[word_start..pos];
            
            if word_start == 0 {
                // First word - complete commands
                (word_start, self.complete_command(word))
            } else {
                // Subsequent words - complete files/paths
                (word_start, self.complete_path(word))
            }
        };

        Ok((start, candidates))
    }
}

impl Hinter for TerminalCompleter {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        if pos < line.len() {
            return None;
        }

        let suggestions = self.get_suggestions(line);
        if suggestions.is_empty() {
            None
        } else {
            Some(suggestions[0].clone())
        }
    }
}

impl Highlighter for TerminalCompleter {
    fn highlight<'l>(&self, line: &'l str, _pos: usize) -> std::borrow::Cow<'l, str> {
        self.bracket_highlighter.highlight(line, _pos)
    }

    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        _default: bool,
    ) -> std::borrow::Cow<'b, str> {
        use colored::*;
        std::borrow::Cow::Owned(prompt.green().to_string())
    }

    fn highlight_char(&self, line: &str, _pos: usize) -> bool {
        !line.is_empty()
    }
}

impl Validator for TerminalCompleter {
    fn validate(&self, _ctx: &mut ValidationContext) -> LineResult<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }

    fn validate_while_typing(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_completion() {
        let completer = TerminalCompleter::new();
        let completions = completer.complete_command("ls");
        assert!(completions.contains(&"ls".to_string()));
        assert!(completions.contains(&"ls".to_string()));
    }

    #[test]
    fn test_alias_completion() {
        let completer = TerminalCompleter::new();
        let completions = completer.complete_command("ll");
        assert!(completions.contains(&"ll".to_string()));
    }

    #[test]
    fn test_get_suggestions() {
        let completer = TerminalCompleter::new();
        let suggestions = completer.get_suggestions("ls");
        assert!(!suggestions.is_empty());
    }
} 
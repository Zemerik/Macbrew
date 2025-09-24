use std::collections::HashMap;
use std::sync::Arc;
use anyhow::Result;

pub struct CommandRegistry {
    commands: HashMap<String, Arc<dyn Command + Send + Sync>>,
}

pub trait Command {
    fn execute(&self, args: &[&str]) -> Result<()>;
    fn help(&self) -> &str;
}

impl CommandRegistry {
    pub fn new() -> Self {
        let mut registry = CommandRegistry {
            commands: HashMap::new(),
        };
        
        // Register built-in commands
        registry.register("ls", Arc::new(LsCommand));
        registry.register("cd", Arc::new(CdCommand));
        registry.register("pwd", Arc::new(PwdCommand));
        registry.register("cat", Arc::new(CatCommand));
        registry.register("echo", Arc::new(EchoCommand));
        registry.register("date", Arc::new(DateCommand));
        registry.register("whoami", Arc::new(WhoamiCommand));
        registry.register("history", Arc::new(HistoryCommand));
        registry.register("help", Arc::new(HelpCommand));
        registry.register("python", Arc::new(PythonCommand));
        registry.register("brew", Arc::new(BrewCommand));
        registry.register("git", Arc::new(GitCommand));
        registry.register("ps", Arc::new(PsCommand));
        registry.register("top", Arc::new(TopCommand));
        registry.register("mkdir", Arc::new(MkdirCommand));
        registry.register("rm", Arc::new(RmCommand));
        registry.register("cp", Arc::new(CpCommand));
        registry.register("mv", Arc::new(MvCommand));
        registry.register("grep", Arc::new(GrepCommand));
        registry.register("find", Arc::new(FindCommand));
        registry.register("chmod", Arc::new(ChmodCommand));
        registry.register("sudo", Arc::new(SudoCommand));
        
        registry
    }

    pub fn register(&mut self, name: &str, command: Arc<dyn Command + Send + Sync>) {
        self.commands.insert(name.to_string(), command);
    }

    pub fn get(&self, name: &str) -> Option<&Arc<dyn Command + Send + Sync>> {
        self.commands.get(name)
    }

    pub fn list_commands(&self) -> Vec<&String> {
        self.commands.keys().collect()
    }
}

// Built-in command implementations
struct LsCommand;
struct CdCommand;
struct PwdCommand;
struct CatCommand;
struct EchoCommand;
struct DateCommand;
struct WhoamiCommand;
struct HistoryCommand;
struct HelpCommand;
struct PythonCommand;
struct BrewCommand;
struct GitCommand;
struct PsCommand;
struct TopCommand;
struct MkdirCommand;
struct RmCommand;
struct CpCommand;
struct MvCommand;
struct GrepCommand;
struct FindCommand;
struct ChmodCommand;
struct SudoCommand;

impl Command for LsCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("ls: List directory contents");
        Ok(())
    }

    fn help(&self) -> &str {
        "List directory contents"
    }
}

impl Command for CdCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("cd: Change directory");
        Ok(())
    }

    fn help(&self) -> &str {
        "Change current directory"
    }
}

impl Command for PwdCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("pwd: Print working directory");
        Ok(())
    }

    fn help(&self) -> &str {
        "Print current working directory"
    }
}

impl Command for CatCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("cat: Display file contents");
        Ok(())
    }

    fn help(&self) -> &str {
        "Display file contents"
    }
}

impl Command for EchoCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("echo: Print text");
        Ok(())
    }

    fn help(&self) -> &str {
        "Print text to stdout"
    }
}

impl Command for DateCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("date: Show current date and time");
        Ok(())
    }

    fn help(&self) -> &str {
        "Display current date and time"
    }
}

impl Command for WhoamiCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("whoami: Show current user");
        Ok(())
    }

    fn help(&self) -> &str {
        "Display current username"
    }
}

impl Command for HistoryCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("history: Show command history");
        Ok(())
    }

    fn help(&self) -> &str {
        "Display command history"
    }
}

impl Command for HelpCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("help: Show available commands");
        Ok(())
    }

    fn help(&self) -> &str {
        "Show help information"
    }
}

impl Command for PythonCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("python: Run Python interpreter or script");
        Ok(())
    }

    fn help(&self) -> &str {
        "Execute Python code or script"
    }
}

impl Command for BrewCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("brew: Homebrew package manager");
        Ok(())
    }

    fn help(&self) -> &str {
        "Homebrew package manager for macOS"
    }
}

impl Command for GitCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("git: Git version control system");
        Ok(())
    }

    fn help(&self) -> &str {
        "Git distributed version control system"
    }
}

impl Command for PsCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("ps: Show running processes");
        Ok(())
    }

    fn help(&self) -> &str {
        "Display running processes"
    }
}

impl Command for TopCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("top: Show system information");
        Ok(())
    }

    fn help(&self) -> &str {
        "Display system information and processes"
    }
}

impl Command for MkdirCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("mkdir: Create directory");
        Ok(())
    }

    fn help(&self) -> &str {
        "Create a new directory"
    }
}

impl Command for RmCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("rm: Remove file or directory");
        Ok(())
    }

    fn help(&self) -> &str {
        "Remove files or directories"
    }
}

impl Command for CpCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("cp: Copy file");
        Ok(())
    }

    fn help(&self) -> &str {
        "Copy files or directories"
    }
}

impl Command for MvCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("mv: Move file");
        Ok(())
    }

    fn help(&self) -> &str {
        "Move or rename files or directories"
    }
}

impl Command for GrepCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("grep: Search for pattern");
        Ok(())
    }

    fn help(&self) -> &str {
        "Search for patterns in files"
    }
}

impl Command for FindCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("find: Find files");
        Ok(())
    }

    fn help(&self) -> &str {
        "Find files in directory tree"
    }
}

impl Command for ChmodCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("chmod: Change file permissions");
        Ok(())
    }

    fn help(&self) -> &str {
        "Change file permissions"
    }
}

impl Command for SudoCommand {
    fn execute(&self, _args: &[&str]) -> Result<()> {
        println!("sudo: Execute command as superuser");
        Ok(())
    }

    fn help(&self) -> &str {
        "Execute command with superuser privileges"
    }
} 
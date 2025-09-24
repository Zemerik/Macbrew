use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::io::{self, Write};
use std::fs;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};

mod commands;
mod history;
mod config;
mod autocomplete;
mod jobs;
mod plugins;
mod utils;

use commands::CommandRegistry;
use history::CommandHistory;
use config::TerminalConfig;
use autocomplete::TerminalCompleter;
use jobs::JobManager;
use plugins::PluginManager;

#[derive(Debug, Serialize, Deserialize)]
struct TerminalSession {
    id: String,
    start_time: DateTime<Utc>,
    commands: Vec<String>,
    current_dir: String,
}

struct TerminalEmulator {
    config: TerminalConfig,
    history: CommandHistory,
    command_registry: CommandRegistry,
    completer: TerminalCompleter,
    job_manager: JobManager,
    plugin_manager: PluginManager,
    current_dir: PathBuf,
    session: TerminalSession,
    environment: HashMap<String, String>,
}

impl TerminalEmulator {
    fn new() -> Result<Self> {
        let config = TerminalConfig::load()?;
        let history = CommandHistory::new();
        let command_registry = CommandRegistry::new();
        let completer = TerminalCompleter::new();
        let job_manager = JobManager::new();
        let mut plugin_manager = PluginManager::new()?;
        plugin_manager.load_plugins()?;
        let current_dir = env::current_dir()?;
        
        let session = TerminalSession {
            id: Uuid::new_v4().to_string(),
            start_time: Utc::now(),
            commands: Vec::new(),
            current_dir: current_dir.to_string_lossy().to_string(),
        };

        let mut environment = HashMap::new();
        for (key, value) in env::vars() {
            environment.insert(key, value);
        }

        Ok(TerminalEmulator {
            config,
            history,
            command_registry,
            completer,
            job_manager,
            plugin_manager,
            current_dir,
            session,
            environment,
        })
    }

    fn run(&mut self) -> Result<()> {
        println!("🚀 Macbrew v1.0.1");
        println!("Type 'help' for available commands, 'exit' to quit");
        println!("Current directory: {}", self.current_dir.display());
        println!();

        loop {
            self.display_prompt();
            
            let input = self.read_input()?;
            if input.trim().is_empty() {
                continue;
            }

            self.session.commands.push(input.clone());
            self.history.add_command(&input);

            if let Err(e) = self.execute_command(&input) {
                eprintln!("Error: {}", e);
            }
        }
    }

    fn display_prompt(&self) {
        let localhost = "localhost".to_string();
        let hostname = self.environment.get("HOSTNAME")
            .or(self.environment.get("COMPUTERNAME"))
            .unwrap_or(&localhost);
        
        let user = "user".to_string();
        let username = self.environment.get("USER")
            .or(self.environment.get("USERNAME"))
            .unwrap_or(&user);

        let current_dir = self.current_dir.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("~");

        print!("{}@{}:{} $ ", username, hostname, current_dir);
        io::stdout().flush().unwrap();
    }

    fn read_input(&self) -> Result<String> {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    fn execute_command(&mut self, input: &str) -> Result<()> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(());
        }

        let command = parts[0];
        let args = &parts[1..];

        match command {
            // Native implementations
            "exit" | "quit" => {
                println!("Goodbye! 👋");
                std::process::exit(0);
            }
            "clear" => {
                print!("\x1B[2J\x1B[1;1H");
                io::stdout().flush()?;
            }
            "pwd" => {
                println!("{}", self.current_dir.display());
            }
            "cd" => {
                self.change_directory(args)?;
            }
            "ls" | "dir" => {
                self.list_directory(args)?;
            }
            "cat" => {
                self.cat_file(args)?;
            }
            "echo" => {
                self.echo(args)?;
            }
            "date" => {
                self.show_date()?;
            }
            "whoami" => {
                self.whoami()?;
            }
            "history" => {
                self.show_history(args)?;
            }
            "help" => {
                self.show_help()?;
            }
            "python" => {
                self.run_python_command(args)?;
            }
            "brew" => {
                self.simulate_brew_command(args)?;
            }
            "git" => {
                self.simulate_git_command(args)?;
            }
            "ps" => {
                self.show_processes()?;
            }
            "top" => {
                self.show_system_info()?;
            }
            "mkdir" => {
                self.make_directory(args)?;
            }
            "rm" => {
                self.remove_file(args)?;
            }
            "cp" => {
                self.copy_file(args)?;
            }
            "mv" => {
                self.move_file(args)?;
            }
            "grep" => {
                self.grep_search(args)?;
            }
            "find" => {
                self.find_files(args)?;
            }
            "chmod" => {
                self.change_permissions(args)?;
            }
            "sudo" => {
                self.sudo_command(args)?;
            }
            "jobs" => {
                self.job_manager.print_jobs();
            }
            "fg" => {
                if args.is_empty() {
                    eprintln!("fg: no current job");
                } else {
                    let job_id = args[0].parse::<usize>()?;
                    self.job_manager.bring_to_foreground(job_id)?;
                }
            }
            "bg" => {
                if args.is_empty() {
                    eprintln!("bg: no current job");
                } else {
                    let job_id = args[0].parse::<usize>()?;
                    self.job_manager.resume_job(job_id)?;
                }
            }
            "kill" => {
                if args.is_empty() {
                    eprintln!("kill: no job specified");
                } else {
                    let job_id = args[0].parse::<usize>()?;
                    self.job_manager.kill_job(job_id)?;
                }
            }
            "alias" | "unalias" | "export" | "unset" | "env" | "which" | "whereis" | "type" | "hash" | "plugins" | "install-plugin" | "uninstall-plugin" | "wc" | "head" | "tail" | "sort" | "uniq" | "cut" | "paste" | "tr" | "sed" | "awk" | "file" | "stat" | "chown" | "chgrp" | "ln" | "touch" | "pkill" | "nohup" | "nice" | "renice" | "systemctl" | "service" | "init" | "systemd-analyze" | "useradd" | "userdel" | "usermod" | "passwd" | "groupadd" | "groupdel" | "id" | "who" | "w" | "df" | "du" | "mount" | "umount" | "fdisk" | "parted" | "mkfs" | "fsck" | "ping" | "ping6" | "traceroute" | "mtr" | "nslookup" | "dig" | "host" | "ifconfig" | "ip" | "netstat" | "ss" | "route" | "arp" | "curl" | "wget" | "ssh" | "scp" | "rsync" | "telnet" | "nc" | "socat" | "nmap" | "tcpdump" | "wireshark" | "iftop" | "nethogs" | "iotop" | "md5sum" | "sha1sum" | "sha256sum" | "sha512sum" | "cksum" | "openssl" | "gpg" | "base64" | "hexdump" | "xxd" | "ssh-keygen" | "ssh-copy-id" | "ssh-add" | "ssh-agent" | "pip" | "conda" | "npm" | "yarn" | "docker" | "kubectl" | "helm" | "terraform" | "ansible" | "ansible-playbook" | "puppet" | "chef" | "vagrant" | "virtualbox" | "vmware" | "qemu" | "kvm" | "libvirt" | "virsh" | "virt-manager" | "vim" | "nano" | "emacs" | "ed" | "less" | "more" | "most" | "gzip" | "gunzip" | "bzip2" | "bunzip2" | "xz" | "unxz" | "tar" | "zip" | "unzip" | "7z" | "rar" | "unrar" | "dmesg" | "journalctl" | "logwatch" | "rsyslog" | "logrotate" | "screen" | "tmux" | "byobu" | "config" | "man" | "info" | "apropos" | "whatis" | "brew" | "pip" | "npm" | "yarn" | "docker" | "kubectl" | "helm" | "terraform" | "ansible" | "puppet" | "chef" | "vagrant" | "virtualbox" | "vmware" | "qemu" | "kvm" | "libvirt" | "virsh" | "virt-manager" => {
                self.execute_external_command(command, args)?;
            }
            _ => {
                println!("Command not found: {}. Type 'help' for available commands.", command);
            }
        }
        Ok(())
    }

    fn change_directory(&mut self, args: &[&str]) -> Result<()> {
        let target = if args.is_empty() {
            self.environment.get("HOME")
                .map(|home| home.as_str())
                .unwrap_or("/")
        } else {
            args[0]
        };

        let new_path = if target.starts_with('/') {
            PathBuf::from(target)
        } else {
            self.current_dir.join(target)
        };

        if new_path.exists() && new_path.is_dir() {
            self.current_dir = fs::canonicalize(new_path)?;
            env::set_current_dir(&self.current_dir)?;
        } else {
            eprintln!("cd: {}: No such file or directory", target);
        }

        Ok(())
    }

    fn list_directory(&self, args: &[&str]) -> Result<()> {
        let path = if args.is_empty() {
            &self.current_dir
        } else {
            &PathBuf::from(args[0])
        };

        if !path.exists() {
            eprintln!("ls: {}: No such file or directory", path.display());
            return Ok(());
        }

        let entries = fs::read_dir(path)?;
        for entry in entries {
            if let Ok(entry) = entry {
                let file_type = if entry.file_type()?.is_dir() { "d" } else { "-" };
                let name = entry.file_name().to_string_lossy().to_string();
                println!("{} {}", file_type, name);
            }
        }

        Ok(())
    }

    fn cat_file(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            eprintln!("cat: missing file operand");
            return Ok(());
        }

        for file_path in args {
            let path = PathBuf::from(file_path);
            if path.exists() {
                let content = fs::read_to_string(&path)?;
                print!("{}", content);
            } else {
                eprintln!("cat: {}: No such file or directory", file_path);
            }
        }

        Ok(())
    }

    fn echo(&self, args: &[&str]) -> Result<()> {
        println!("{}", args.join(" "));
        Ok(())
    }

    fn show_date(&self) -> Result<()> {
        let now: DateTime<Utc> = Utc::now();
        println!("{}", now.format("%a %b %d %H:%M:%S %Z %Y"));
        Ok(())
    }

    fn whoami(&self) -> Result<()> {
        let unknown = "unknown".to_string();
        let username = self.environment.get("USER")
            .or(self.environment.get("USERNAME"))
            .unwrap_or(&unknown);
        println!("{}", username);
        Ok(())
    }

    fn show_history(&self, args: &[&str]) -> Result<()> {
        let limit = if !args.is_empty() {
            args[0].parse::<usize>().unwrap_or(10)
        } else {
            10
        };

        let commands = self.history.get_recent(limit);
        for (i, cmd) in commands.iter().enumerate() {
            println!("{}: {}", i + 1, cmd);
        }

        Ok(())
    }

    fn show_help(&self) -> Result<()> {
        println!("Available commands:");
        println!("  ls, dir          - List directory contents");
        println!("  cd <directory>   - Change directory");
        println!("  pwd              - Print working directory");
        println!("  cat <file>       - Display file contents");
        println!("  echo <text>      - Print text");
        println!("  date             - Show current date/time");
        println!("  whoami           - Show current user");
        println!("  history [n]      - Show command history");
        println!("  clear            - Clear screen");
        println!("  mkdir <dir>      - Create directory");
        println!("  rm <file>        - Remove file");
        println!("  cp <src> <dest>  - Copy file");
        println!("  mv <src> <dest>  - Move file");
        println!("  grep <pattern>   - Search for pattern");
        println!("  find <path>      - Find files");
        println!("  chmod <mode>     - Change permissions");
        println!("  ps               - Show processes");
        println!("  top              - Show system info");
        println!("  python <script>  - Run Python script");
        println!("  brew <command>   - Homebrew package manager");
        println!("  git <command>    - Git version control");
        println!("  sudo <command>   - Run as superuser");
        println!("  help             - Show this help");
        println!("  exit, quit       - Exit terminal");
        Ok(())
    }

    fn run_python_command(&self, args: &[&str]) -> Result<()> {
        use std::process::Command;
        if args.is_empty() {
            // Launch interactive python shell
            let mut child = Command::new("python3")
                .spawn()
                .context("Failed to launch Python interpreter")?;
            let status = child.wait()?;
            println!("Python exited with status: {}", status);
            return Ok(());
        }

        let output = Command::new("python3")
            .args(args)
            .output()
            .context("Failed to execute Python script")?;

        if !output.stdout.is_empty() {
            print!("{}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            eprint!("{}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(())
    }

    fn simulate_brew_command(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Example usage:");
            println!("  brew search [TEXT|/REGEX/]");
            println!("  brew info [FORMULA...]");
            println!("  brew install FORMULA...");
            println!("  brew update");
            println!("  brew upgrade [FORMULA...]");
            println!("  brew uninstall FORMULA...");
            return Ok(());
        }

        match args[0] {
            "install" => {
                if args.len() > 1 {
                    println!("==> Installing {}...", args[1]);
                    println!("🍺  /usr/local/Cellar/{}/1.0.0: 10 files, 1.2MB", args[1]);
                } else {
                    eprintln!("Error: No formula specified for install");
                }
            }
            "search" => {
                if args.len() > 1 {
                    println!("==> Searching for \"{}\"...", args[1]);
                    println!("Found: {} (formula)", args[1]);
                } else {
                    eprintln!("Error: No search term specified");
                }
            }
            "update" => {
                println!("==> Updating Homebrew...");
                println!("Updated 2 taps (homebrew/core, homebrew/cask).");
            }
            "upgrade" => {
                println!("==> Upgrading packages...");
                println!("All packages are up to date.");
            }
            _ => {
                println!("Unknown brew command: {}", args[0]);
            }
        }

        Ok(())
    }

    fn simulate_git_command(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("usage: git [--version] [--help] [-C <path>] [-c name=value]");
            println!("           [--exec-path[=<path>]] [--html-path] [--man-path] [--info-path]");
            println!("           [-p | --paginate | -P | --no-pager] [--no-replace-objects] [--bare]");
            println!("           [--git-dir=<path>] [--work-tree=<path>] [--namespace=<name>]");
            println!("           <command> [<args>]");
            return Ok(());
        }

        match args[0] {
            "status" => {
                println!("On branch main");
                println!("Your branch is up to date with 'origin/main'.");
                println!("nothing to commit, working tree clean");
            }
            "log" => {
                println!("commit abc123def456 (HEAD -> main)");
                println!("Author: User <user@example.com>");
                println!("Date:   Mon Jan 1 12:00:00 2025 +0000");
                println!("");
                println!("    Initial commit");
            }
            "add" => {
                if args.len() > 1 {
                    println!("Added '{}' to staging area", args[1]);
                } else {
                    println!("Added all files to staging area");
                }
            }
            "commit" => {
                println!("[main abc123d] Commit message");
                println!(" 1 file changed, 1 insertion(+)");
            }
            "push" => {
                println!("Enumerating objects: 3, done.");
                println!("Counting objects: 100% (3/3), done.");
                println!("Delta compression using up to 8 threads");
                println!("Compressing objects: 100% (2/2), done.");
                println!("Writing objects: 100% (3/3), done.");
                println!("Total 3 (delta 0), reused 0 (delta 0), pack-reused 0");
                println!("To https://github.com/user/repo.git");
                println!("   abc123..def456  main -> main");
            }
            "pull" => {
                println!("From https://github.com/user/repo.git");
                println!("   abc123..def456  main     -> origin/main");
                println!("Updating abc123..def456");
                println!("Fast-forward");
            }
            "clone" => {
                if args.len() > 1 {
                    println!("Cloning into '{}'...", args[1]);
                    println!("remote: Enumerating objects: 100, done.");
                    println!("remote: Counting objects: 100% (100/100), done.");
                    println!("remote: Compressing objects: 100% (80/80), done.");
                    println!("Receiving objects: 100% (100/100), done.");
                    println!("Resolving deltas: 100% (50/50), done.");
                } else {
                    eprintln!("Error: No repository URL specified");
                }
            }
            _ => {
                println!("Unknown git command: {}", args[0]);
            }
        }

        Ok(())
    }

    fn show_processes(&self) -> Result<()> {
        println!("  PID TTY          TIME CMD");
        println!(" 1234 pts/0    00:00:01 bash");
        println!(" 1235 pts/0    00:00:00 ps");
        println!(" 1236 pts/0    00:00:02 python");
        Ok(())
    }

    fn show_system_info(&self) -> Result<()> {
        println!("Processes: 3 total, 2 running, 1 sleeping");
        println!("CPU usage: 15% user, 5% system, 80% idle");
        println!("Memory: 8GB total, 4GB used, 4GB free");
        println!("Load average: 0.5, 0.3, 0.2");
        Ok(())
    }

    fn make_directory(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            eprintln!("mkdir: missing operand");
            return Ok(());
        }

        for dir_name in args {
            let path = PathBuf::from(dir_name);
            if path.exists() {
                eprintln!("mkdir: cannot create directory '{}': File exists", dir_name);
            } else {
                fs::create_dir(&path)?;
                println!("Created directory: {}", dir_name);
            }
        }

        Ok(())
    }

    fn remove_file(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            eprintln!("rm: missing operand");
            return Ok(());
        }

        for file_name in args {
            let path = PathBuf::from(file_name);
            if path.exists() {
                fs::remove_file(&path)?;
                println!("Removed: {}", file_name);
            } else {
                eprintln!("rm: cannot remove '{}': No such file or directory", file_name);
            }
        }

        Ok(())
    }

    fn copy_file(&self, args: &[&str]) -> Result<()> {
        if args.len() < 2 {
            eprintln!("cp: missing file operand");
            return Ok(());
        }

        let source = PathBuf::from(args[0]);
        let dest = PathBuf::from(args[1]);

        if !source.exists() {
            eprintln!("cp: cannot stat '{}': No such file or directory", args[0]);
            return Ok(());
        }

        fs::copy(&source, &dest)?;
        println!("Copied '{}' to '{}'", args[0], args[1]);

        Ok(())
    }

    fn move_file(&self, args: &[&str]) -> Result<()> {
        if args.len() < 2 {
            eprintln!("mv: missing file operand");
            return Ok(());
        }

        let source = PathBuf::from(args[0]);
        let dest = PathBuf::from(args[1]);

        if !source.exists() {
            eprintln!("mv: cannot stat '{}': No such file or directory", args[0]);
            return Ok(());
        }

        fs::rename(&source, &dest)?;
        println!("Moved '{}' to '{}'", args[0], args[1]);

        Ok(())
    }

    fn grep_search(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            eprintln!("grep: missing pattern");
            return Ok(());
        }

        let pattern = args[0];
        let files = &args[1..];

        if files.is_empty() {
            println!("Searching for '{}' in current directory...", pattern);
            println!("(Simulated grep output)");
        } else {
            for file in files {
                let path = PathBuf::from(file);
                if path.exists() {
                    println!("{}: Found pattern '{}' in file", file, pattern);
                } else {
                    eprintln!("grep: {}: No such file or directory", file);
                }
            }
        }

        Ok(())
    }

    fn find_files(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            println!("Searching in current directory...");
            println!("./file1.txt");
            println!("./file2.py");
            println!("./directory/");
            return Ok(());
        }

        let search_path = args[0];
        let path = PathBuf::from(search_path);
        
        if path.exists() {
            println!("Searching in '{}'...", search_path);
            println!("{}/result1.txt", search_path);
            println!("{}/result2.py", search_path);
        } else {
            eprintln!("find: '{}': No such file or directory", search_path);
        }

        Ok(())
    }

    fn change_permissions(&self, args: &[&str]) -> Result<()> {
        if args.len() < 2 {
            eprintln!("chmod: missing operand");
            return Ok(());
        }

        let mode = args[0];
        let file = args[1];

        println!("Changed permissions of '{}' to {}", file, mode);
        Ok(())
    }

    fn sudo_command(&self, args: &[&str]) -> Result<()> {
        if args.is_empty() {
            eprintln!("sudo: no command specified");
            return Ok(());
        }

        println!("[sudo] password for user: ");
        println!("(Simulated sudo execution)");
        println!("Running: {}", args.join(" "));
        Ok(())
    }

    fn execute_external_command(&self, command: &str, args: &[&str]) -> Result<()> {
        let output_result = Command::new(command)
            .args(args)
            .current_dir(&self.current_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();

        match output_result {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    io::stdout().write_all(&output.stdout)?;
                }
                if !output.stderr.is_empty() {
                    io::stderr().write_all(&output.stderr)?;
                }
            }
            Err(e) => {
                eprintln!("\n[Macbrew Terminal Emulator]\nError: Failed to execute external command '{}'.\nThis usually means the command is not installed, not in your PATH, or not available on your system.\n\nTroubleshooting steps:\n  1. Check for typos in the command.\n  2. Make sure the command is installed on your system.\n  3. Ensure the command's directory is in your PATH environment variable.\n  4. Try running the command in your system terminal (e.g., Command Prompt, PowerShell, or Terminal).\n  5. If it works there but not here, please report an issue.\n\nSystem error: {}\n", command, e);
            }
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut terminal = TerminalEmulator::new()?;
    terminal.run()
} 
![Macbrew](assets/logo1.png)

A feature-rich terminal emulator built in Rust with Python integration, designed to provide a comprehensive command-line experience similar to macOS/Homebrew environments.

[![npm version](https://img.shields.io/npm/v/@zemerik/macbrew.svg)](https://www.npmjs.com/package/@zemerik/macbrew)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Python](https://img.shields.io/badge/Python-3.8+-blue.svg)](https://www.python.org/)

## ✨ Features

### 🎯 Core Features
- **Real Python Execution**: Execute Python scripts and interactive Python sessions
- **Command History**: Persistent command history with search and navigation
- **Autocomplete**: Intelligent command and file path completion
- **Job Control**: Background process management with `jobs`, `fg`, `bg`, `kill`
- **Plugin System**: Extensible plugin architecture for custom commands
- **Color Support**: Rich colored output and syntax highlighting
- **Configuration**: Customizable settings and aliases

### 📁 File Operations
- **Basic Commands**: `ls`, `cd`, `pwd`, `cat`, `echo`, `mkdir`, `rm`, `cp`, `mv`
- **Advanced File Operations**: `find`, `grep`, `chmod`, `chown`, `chgrp`
- **Text Processing**: `head`, `tail`, `sort`, `uniq`, `cut`, `paste`, `tr`, `sed`, `awk`
- **File Analysis**: `wc`, `file`, `stat`, `du`, `df`

### 🔧 System Administration
- **Process Management**: `ps`, `top`, `htop`, `kill`, `nohup`
- **System Services**: `systemctl`, `service`, `init`
- **User Management**: `useradd`, `userdel`, `groupadd`, `groupdel`, `passwd`
- **Disk Management**: `fdisk`, `parted`, `mkfs`, `fsck`, `mount`, `umount`
- **Scheduling**: `cron`, `at`, `batch`, `crontab`

### 🌐 Networking
- **Network Tools**: `ping`, `curl`, `wget`, `ssh`, `scp`, `rsync`
- **Network Analysis**: `netstat`, `lsof`, `nmap`, `tcpdump`
- **Network Monitoring**: `iftop`, `nethogs`, `iotop`
- **Remote Access**: `telnet`, `nc`, `socat`

### 🔐 Security & Cryptography
- **Encryption**: `openssl`, `gpg`, `ssh-keygen`
- **Hashing**: `md5sum`, `sha256sum`, `sha512sum`
- **Encoding**: `base64`, `hexdump`, `xxd`

### 📦 Package Management
- **Homebrew**: Full Homebrew package manager simulation
- **Python**: `pip`, `conda` package management
- **Node.js**: `npm`, `yarn` package management
- **Docker**: Container management commands

### 🐳 DevOps & Cloud
- **Containerization**: `docker`, `kubectl`, `helm`
- **Infrastructure**: `terraform`, `ansible`, `puppet`, `chef`
- **Virtualization**: `vagrant`, `virtualbox`, `vmware`, `qemu`, `kvm`
- **Cloud Tools**: `libvirt`, `virsh`, `virt-manager`

### 📝 Text Editors
- **Editors**: `vim`, `nano`, `emacs`
- **Pagers**: `less`, `more`

### 🔍 Documentation
- **Manual Pages**: `man`, `info`, `apropos`, `whatis`
- **Help System**: Built-in help for all commands

### 📦 Compression & Archives
- **Compression**: `gzip`, `gunzip`, `bzip2`, `bunzip2`, `xz`, `unxz`
- **Archives**: `tar`, `zip`, `unzip`, `7z`, `rar`, `unrar`

### 📊 Logging & Monitoring
- **System Logs**: `dmesg`, `syslog`, `rsyslog`, `logwatch`
- **Journal**: `journalctl`, `logrotate`

### 🎮 Terminal Multiplexing
- **Sessions**: `screen`, `tmux`

## 🚀 Quick Start

### Prerequisites
- **Rust** (1.70 or higher) - [Install Rust](https://rustup.rs/)
- **Python 3.8+** - [Install Python](https://www.python.org/downloads/)
- **Node.js 14+** - [Install Node.js](https://nodejs.org/)

> [!CAUTION]
> **⚠️ Windows Users:** If you encounter a `link.exe not found` error when building, you need to install Visual Studio Build Tools with the "Desktop development with C++" workload. Alternatively, you can switch to the GNU toolchain by running:
> ```bash
> rustup default stable-x86_64-pc-windows-gnu
> ```
> For the GNU toolchain, you may also need to install [MinGW-w64](https://www.mingw-w64.org/).

### Installation

#### Option 1: Using npx (Recommended)
```bash
npx macbrew
```

#### Option 2: Global Installation
```bash
npm install -g macbrew
macbrew
```

#### Option 3: From Source
```bash
git clone https://github.com/Zemerik/Macbrew.git
cd Macbrew
cargo build --release
cargo run
```

## 📖 Usage Examples

### Basic Commands
```bash
# File operations
ls -la
cd /path/to/directory
pwd
cat filename.txt
echo "Hello, World!"

# Python execution
python -c "print('Hello from Python!')"
python script.py

# Homebrew package management
brew install package_name
brew search query
brew update
brew upgrade

# Plugin commands (included by default)
hello World
weather Tokyo
```

### Advanced Features
```bash
# Job control
long_running_command &
jobs
fg 1
bg 1
kill 1

# Plugin management
plugins list
install-plugin example
uninstall-plugin example

# Text processing
echo "hello world" | tr '[:lower:]' '[:upper:]'
cat file.txt | sort | uniq
grep "pattern" file.txt
```

### System Administration
```bash
# Process management
ps aux
top
kill -9 process_id

# User management
useradd username
passwd username
userdel username

# System services
systemctl status service_name
systemctl start service_name
systemctl stop service_name
```

### Networking
```bash
# Network diagnostics
ping google.com
curl https://api.github.com
ssh user@hostname
scp file.txt user@hostname:/path/

# Network analysis
netstat -tuln
lsof -i :8080
nmap localhost
```

### Security
```bash
# Hashing
echo "password" | md5sum
echo "password" | sha256sum

# Encryption
openssl enc -aes-256-cbc -in file.txt -out file.enc
gpg --encrypt file.txt

# SSH key management
ssh-keygen -t rsa -b 4096
```

## ⚙️ Configuration

The terminal emulator uses a configuration file located at:
- **Linux/macOS**: `~/.config/terminal-emulator/config.json`
- **Windows**: `%APPDATA%\terminal-emulator\config.json`

### Configuration Options
```json
{
  "prompt_style": {
    "show_hostname": true,
    "show_username": true,
    "show_path": true,
    "show_git_branch": false,
    "format": "{username}@{hostname}:{path} $ "
  },
  "history_size": 1000,
  "auto_completion": true,
  "syntax_highlighting": true,
  "colors": {
    "prompt": "\u001b[32m",
    "command": "\u001b[36m",
    "output": "\u001b[0m",
    "error": "\u001b[31m",
    "success": "\u001b[32m",
    "warning": "\u001b[33m"
  }
}
```

## 🔌 Plugin System

The terminal emulator supports a plugin system for extending functionality:

### Creating a Plugin
1. Create a plugin directory in `~/.config/terminal-emulator/plugins/`
2. Add a `plugin.toml` manifest file
3. Include your script files

### Example Plugin
```toml
# plugin.toml
name = "my-plugin"
version = "1.0.0"
description = "My custom plugin"
author = "Your Name"
commands = [
    { name = "hello", description = "Say hello", usage = "hello [name]", script = "hello.py", language = "python", enabled = true }
]
```

```python
# hello.py
#!/usr/bin/env python3
import sys

def main():
    if len(sys.argv) > 1:
        name = sys.argv[1]
        print(f"Hello, {name}!")
    else:
        print("Hello, World!")

if __name__ == "__main__":
    main()
```

## 🎨 Customization

### Aliases
```bash
# Set aliases
alias ll='ls -la'
alias g='git'
alias p='python'

# List aliases
alias

# Remove aliases
unalias ll
```

### Environment Variables
```bash
# Set environment variables
export EDITOR=vim
export PATH=$PATH:/custom/path

# Show environment
env

# Unset variables
unset VARIABLE_NAME
```

## 🔧 Development

### Project Structure
```
macbrew/
├── src/                    # TypeScript source code
│   ├── bin/               # Binary launcher
│   │   └── macbrew.ts     # Main binary launcher
│   ├── scripts/           # Installation scripts
│   │   ├── install.ts     # Build and setup script
│   │   ├── postinstall.ts # Post-installation setup
│   │   └── uninstall.ts   # Cleanup script
│   ├── types/             # TypeScript type definitions
│   │   ├── config.ts      # Configuration types
│   │   ├── plugin.ts      # Plugin types
│   │   └── terminal.ts    # Terminal types
│   ├── lib/               # Core library modules
│   │   ├── macbrew.ts     # Main Macbrew class
│   │   ├── terminal.ts    # Terminal functionality
│   │   ├── plugin-manager.ts # Plugin system
│   │   └── config-manager.ts # Configuration management
│   └── index.ts           # Main entry point
├── dist/                  # Compiled JavaScript output
├── bin/                   # JavaScript wrappers
│   └── macbrew.js         # Binary launcher wrapper
├── scripts/               # JavaScript wrappers
│   ├── install.js         # Install script wrapper
│   ├── postinstall.js     # Postinstall script wrapper
│   └── uninstall.js       # Uninstall script wrapper
├── package.json           # NPM package configuration
├── Cargo.toml            # Rust dependencies
└── README.md             # This file
```

### Adding New Commands
1. Add the command to the `execute_command` match statement in `main.rs`
2. Implement the command function
3. Add help documentation

### Building for Distribution
```bash
# Build optimized release
cargo build --release

# Create installation package
cargo install --path .

# Publish to npm
npm publish
```

## 🐛 Troubleshooting

### Common Issues

1. **Python not found**: Ensure Python 3.x is installed and in PATH
2. **Permission denied**: Check file permissions and ownership
3. **Plugin not loading**: Verify plugin.toml syntax and file paths
4. **Command not found**: Check if the command is available in your system

### Debug Mode
Run with debug output:
```bash
RUST_LOG=debug cargo run
```

## ❗ Troubleshooting Command Not Found or Execution Errors

If you see an error like:

```
Error: Failed to execute external command 'your-command'.
This usually means the command is not installed, not in your PATH, or not available on your system.
```

### What does this mean?
- The command you tried to run is not available on your device, or your system cannot find it.
- This is **not a bug in Macbrew**. Macbrew passes commands to your operating system, so if the OS can't find or run the command, neither can Macbrew.

### How to Fix
1. **Check for typos**: Make sure you typed the command correctly.
2. **Is the command installed?**
   - On Windows, some commands (like `ls`, `grep`, `awk`, etc.) are not available by default. You may need to install tools like Git Bash, Cygwin, or Windows Subsystem for Linux (WSL).
   - On macOS/Linux, use your package manager (e.g., `brew install`, `apt install`, `yum install`) to install missing commands.
3. **Check your PATH**: Make sure the directory containing the command is in your system's PATH environment variable.
4. **Try in your system terminal**: Open Command Prompt, PowerShell, or Terminal and try the command there. If it fails, it's a system issue.
5. **Permissions**: Make sure you have permission to run the command.
6. **Still not working?**
   - If the command works in your system terminal but not in Macbrew, please [open an issue](https://github.com/Zemerik/Macbrew/issues) with details.

### Example
If you type `ls` on Windows and see this error, try using `dir` instead, or install a Unix-like environment.

---

## 📦 NPM Package

Rusty Shell is available as an npm package for easy installation and distribution.

### Package Information
- **Package Name**: `macbrew`
- **Registry**: [npmjs.com](https://www.npmjs.com/package/macbrew)
- **Install Command**: `npm install -g macbrew`
- **NPX Command**: `npx macbrew`

### Package Features
- ✅ Automatic Rust binary compilation
- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Automatic dependency checking
- ✅ Configuration setup
- ✅ Example plugins included
- ✅ Clean uninstallation

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📞 Support

- **Issues**: Report bugs and feature requests on GitHub
- **Documentation**: Check the inline documentation and examples
- **Community**: Join our community discussions

## 🎯 Roadmap

- [ ] GUI mode with TUI interface
- [ ] Remote terminal support
- [ ] Advanced scripting capabilities
- [ ] Package manager integration
- [ ] Cloud provider integrations
- [ ] Machine learning command suggestions
- [ ] Voice command support
- [ ] Advanced debugging tools

---

**Built with ❤️ using Rust, Python, and TypeScript**

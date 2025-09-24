---
layout: ../../../layouts/Layout.astro
title: Installation Guide - Macbrew Documentation
description: Learn how to install Macbrew on Windows, macOS, and Linux
---

# Installation Guide

Get Macbrew up and running on your system in just a few minutes.

## Prerequisites

Before installing Macbrew, make sure you have the following installed:

- **Rust** (1.70 or higher) - [Install Rust](https://rustup.rs/)
- **Python 3.8+** - [Install Python](https://www.python.org/downloads/)
- **Node.js 14+** - [Install Node.js](https://nodejs.org/)

## Quick Installation

### Using npx (Recommended)

The fastest way to try Macbrew is using npx:

```bash
npx macbrew
```

This will automatically download and run the latest version of Macbrew without installing anything permanently.

### Global Installation

For regular use, install Macbrew globally:

```bash
npm install -g macbrew
```

Then run it:

```bash
macbrew
```

## Platform-Specific Instructions

### Windows

1. **Install Rust**:
   ```bash
   # Download and run rustup-init.exe from https://rustup.rs/
   # Or use winget
   winget install Rust.Rust
   ```

2. **Install Python**:
   ```bash
   # Download from https://www.python.org/downloads/
   # Or use winget
   winget install Python.Python.3.11
   ```

3. **Install Node.js**:
   ```bash
   # Download from https://nodejs.org/
   # Or use winget
   winget install OpenJS.NodeJS
   ```

4. **Install Macbrew**:
   ```bash
   npm install -g macbrew
   ```

### macOS

1. **Install Rust**:
   ```bash
   curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install Python** (if not already installed):
   ```bash
   # Using Homebrew
   brew install python@3.11
   
   # Or download from https://www.python.org/downloads/
   ```

3. **Install Node.js**:
   ```bash
   # Using Homebrew
   brew install node
   
   # Or download from https://nodejs.org/
   ```

4. **Install Macbrew**:
   ```bash
   npm install -g macbrew
   ```

### Linux

1. **Install Rust**:
   ```bash
   curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Install Python**:
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install python3 python3-pip
   
   # CentOS/RHEL/Fedora
   sudo dnf install python3 python3-pip
   # or
   sudo yum install python3 python3-pip
   ```

3. **Install Node.js**:
   ```bash
   # Using NodeSource repository (Ubuntu/Debian)
   curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
   sudo apt-get install -y nodejs
   
   # CentOS/RHEL/Fedora
   curl -fsSL https://rpm.nodesource.com/setup_18.x | sudo bash -
   sudo yum install -y nodejs
   ```

4. **Install Macbrew**:
   ```bash
   npm install -g macbrew
   ```

## From Source

If you want to build from source or contribute to the project:

```bash
# Clone the repository
git clone https://github.com/Zemerik/Macbrew.git
cd Macbrew

# Install dependencies
npm install

# Build the project
npm run build

# Run in development mode
npm run dev
```

## Verification

After installation, verify that Macbrew is working correctly:

```bash
# Check if Macbrew is installed
macbrew --version

# Start Macbrew
macbrew

# You should see the welcome message and prompt
```

## First Run

When you run Macbrew for the first time, it will:

1. **Build the Rust binary** (if not already built)
2. **Set up configuration** in `~/.config/terminal-emulator/`
3. **Install example plugins** in `~/.config/terminal-emulator/plugins/`
4. **Show the welcome message**

## Configuration

Macbrew creates a configuration file at:
- **Linux/macOS**: `~/.config/terminal-emulator/config.json`
- **Windows**: `%APPDATA%\terminal-emulator\config.json`

You can customize various settings like:
- Prompt style and colors
- Command history size
- Auto-completion settings
- Plugin configurations

## Troubleshooting

### Common Issues

**"Rust is not installed"**
```bash
# Install Rust
curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**"Python is not found"**
```bash
# Make sure Python 3.8+ is installed and in PATH
python3 --version
# or
python --version
```

**"Permission denied"**
```bash
# On Linux/macOS, you might need to use sudo
sudo npm install -g macbrew

# Or configure npm to use a different directory
npm config set prefix ~/.npm-global
export PATH=~/.npm-global/bin:$PATH
```

**"Build failed"**
```bash
# Clean and rebuild
npm run clean
npm run build
```

### Getting Help

If you encounter any issues:

1. Check the [troubleshooting guide](/docs/troubleshooting)
2. Search [existing issues](https://github.com/Zemerik/Macbrew/issues)
3. Create a [new issue](https://github.com/Zemerik/Macbrew/issues/new)
4. Join our [Discussions](https://github.com/Zemerik/Macbrew/discussions)

## Next Steps

Now that you have Macbrew installed, check out:

- [Getting Started Guide](/docs/getting-started) - Learn the basics
- [Features Overview](/docs/features) - Explore what Macbrew can do
- [Command Reference](/docs/commands) - Browse available commands
- [Plugin System](/docs/plugins) - Extend functionality with plugins 
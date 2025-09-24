---
layout: ../../layouts/Layout.astro
title: Plugin System - Macbrew Documentation
description: Learn how to extend Macbrew with custom plugins
---

# Plugin System

Macbrew's plugin system allows you to extend functionality with custom commands and features.

## Overview

Plugins are modular extensions that add new commands and functionality to Macbrew. They can be written in Python, JavaScript, or any language that can be executed from the command line.

## Plugin Structure

A plugin consists of:

```
plugin-name/
├── plugin.toml          # Plugin manifest
├── commands/            # Command scripts
│   ├── hello.py
│   └── weather.py
├── lib/                 # Library files
│   └── utils.py
└── README.md           # Plugin documentation
```

## Plugin Manifest

The `plugin.toml` file defines the plugin metadata and commands:

```toml
name = "my-plugin"
version = "1.0.0"
description = "My custom plugin for Macbrew"
author = "Your Name"
repository = "https://github.com/username/my-plugin"
license = "MIT"

[commands]
hello = { description = "Say hello", usage = "hello [name]", script = "commands/hello.py", language = "python", enabled = true }
weather = { description = "Get weather info", usage = "weather [city]", script = "commands/weather.py", language = "python", enabled = true }

[config]
api_key = { type = "string", default = "", description = "API key for weather service" }
```

## Creating a Plugin

### Step 1: Create Plugin Directory

```bash
mkdir ~/.config/terminal-emulator/plugins/my-plugin
cd ~/.config/terminal-emulator/plugins/my-plugin
```

### Step 2: Create Plugin Manifest

Create `plugin.toml`:

```toml
name = "my-plugin"
version = "1.0.0"
description = "Example plugin for Macbrew"
author = "Your Name"
license = "MIT"

[commands]
hello = { description = "Say hello to someone", usage = "hello [name]", script = "hello.py", language = "python", enabled = true }
calculator = { description = "Simple calculator", usage = "calculator [expression]", script = "calculator.py", language = "python", enabled = true }

[config]
greeting = { type = "string", default = "Hello", description = "Default greeting message" }
```

### Step 3: Create Command Scripts

Create `hello.py`:

```python
#!/usr/bin/env python3
import sys
import os

def main():
    # Get plugin config
    config_path = os.path.join(os.path.dirname(__file__), '..', 'config.json')
    greeting = "Hello"
    
    if os.path.exists(config_path):
        import json
        with open(config_path, 'r') as f:
            config = json.load(f)
            greeting = config.get('greeting', 'Hello')
    
    # Get name from arguments
    name = sys.argv[1] if len(sys.argv) > 1 else "World"
    
    print(f"{greeting}, {name}!")

if __name__ == "__main__":
    main()
```

Create `calculator.py`:

```python
#!/usr/bin/env python3
import sys

def main():
    if len(sys.argv) < 2:
        print("Usage: calculator [expression]")
        print("Example: calculator '2 + 3 * 4'")
        return
    
    expression = ' '.join(sys.argv[1:])
    
    try:
        # Safe evaluation (only basic math operations)
        allowed_chars = set('0123456789+-*/.() ')
        if not all(c in allowed_chars for c in expression):
            print("Error: Invalid characters in expression")
            return
        
        result = eval(expression)
        print(f"{expression} = {result}")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
```

### Step 4: Test Your Plugin

```bash
# Reload plugins
plugins reload

# Test your commands
hello Alice
calculator "2 + 3 * 4"
```

## Plugin Management Commands

### List Plugins

```bash
plugins list
```

Output:
```
Installed Plugins:
├── my-plugin (1.0.0) - Example plugin for Macbrew
├── weather-plugin (2.1.0) - Weather information
└── git-helper (1.5.0) - Git workflow helpers

Available Commands:
├── hello (my-plugin)
├── calculator (my-plugin)
├── weather (weather-plugin)
└── git-status (git-helper)
```

### Install Plugin

```bash
# Install from local directory
install-plugin /path/to/plugin

# Install from Git repository
install-plugin https://github.com/username/plugin-name

# Install from npm package
install-plugin @username/plugin-name
```

### Enable/Disable Plugin

```bash
# Enable plugin
enable-plugin my-plugin

# Disable plugin
disable-plugin my-plugin
```

### Uninstall Plugin

```bash
uninstall-plugin my-plugin
```

### Reload Plugins

```bash
plugins reload
```

## Plugin Configuration

### Setting Configuration

```bash
# Set plugin configuration
config set my-plugin.greeting "Hi there"

# Get plugin configuration
config get my-plugin.greeting

# List all plugin configurations
config list my-plugin
```

### Configuration File

Plugin configurations are stored in `~/.config/terminal-emulator/plugins/[plugin-name]/config.json`:

```json
{
  "greeting": "Hi there",
  "api_key": "your-api-key-here",
  "debug": false
}
```

## Advanced Plugin Features

### Plugin Dependencies

Specify dependencies in your `plugin.toml`:

```toml
[dependencies]
requests = "2.28.0"
click = "8.1.0"

[dependencies.system]
curl = "7.0.0"
jq = "1.6"
```

### Plugin Hooks

Plugins can define hooks that run at specific events:

```toml
[hooks]
pre_command = "hooks/pre_command.py"
post_command = "hooks/post_command.py"
startup = "hooks/startup.py"
shutdown = "hooks/shutdown.py"
```

Example hook script (`hooks/startup.py`):

```python
#!/usr/bin/env python3
import os

def main():
    print("My plugin is starting up!")
    # Initialize plugin resources
    # Set up API connections
    # Load cached data

if __name__ == "__main__":
    main()
```

### Plugin API

Plugins can access Macbrew's internal API:

```python
#!/usr/bin/env python3
import os
import json

def get_macbrew_config():
    """Get Macbrew configuration"""
    config_path = os.path.expanduser("~/.config/terminal-emulator/config.json")
    if os.path.exists(config_path):
        with open(config_path, 'r') as f:
            return json.load(f)
    return {}

def get_plugin_config(plugin_name):
    """Get plugin-specific configuration"""
    config_path = os.path.expanduser(f"~/.config/terminal-emulator/plugins/{plugin_name}/config.json")
    if os.path.exists(config_path):
        with open(config_path, 'r') as f:
            return json.load(f)
    return {}

def main():
    # Access Macbrew configuration
    macbrew_config = get_macbrew_config()
    
    # Access plugin configuration
    plugin_config = get_plugin_config("my-plugin")
    
    print(f"Macbrew theme: {macbrew_config.get('theme', 'default')}")
    print(f"Plugin greeting: {plugin_config.get('greeting', 'Hello')}")

if __name__ == "__main__":
    main()
```

## Example Plugins

### Weather Plugin

```toml
# plugin.toml
name = "weather"
version = "1.0.0"
description = "Get weather information"
author = "Weather Enthusiast"

[commands]
weather = { description = "Get current weather", usage = "weather [city]", script = "weather.py", language = "python", enabled = true }
forecast = { description = "Get weather forecast", usage = "forecast [city] [days]", script = "forecast.py", language = "python", enabled = true }

[config]
api_key = { type = "string", default = "", description = "OpenWeatherMap API key" }
units = { type = "string", default = "metric", description = "Temperature units (metric/imperial)" }
```

```python
# weather.py
#!/usr/bin/env python3
import sys
import requests
import json
import os

def get_config():
    config_path = os.path.join(os.path.dirname(__file__), 'config.json')
    if os.path.exists(config_path):
        with open(config_path, 'r') as f:
            return json.load(f)
    return {}

def main():
    config = get_config()
    api_key = config.get('api_key')
    units = config.get('units', 'metric')
    
    if not api_key:
        print("Error: API key not configured. Run: config set weather.api_key YOUR_API_KEY")
        return
    
    city = sys.argv[1] if len(sys.argv) > 1 else "London"
    
    url = f"http://api.openweathermap.org/data/2.5/weather"
    params = {
        'q': city,
        'appid': api_key,
        'units': units
    }
    
    try:
        response = requests.get(url, params=params)
        data = response.json()
        
        if response.status_code == 200:
            temp = data['main']['temp']
            description = data['weather'][0]['description']
            humidity = data['main']['humidity']
            
            unit_symbol = "°C" if units == "metric" else "°F"
            print(f"Weather in {city}:")
            print(f"Temperature: {temp}{unit_symbol}")
            print(f"Description: {description}")
            print(f"Humidity: {humidity}%")
        else:
            print(f"Error: {data.get('message', 'Unknown error')}")
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()
```

### Git Helper Plugin

```toml
# plugin.toml
name = "git-helper"
version = "1.0.0"
description = "Git workflow helpers"
author = "Git Enthusiast"

[commands]
git-status = { description = "Enhanced git status", usage = "git-status", script = "git_status.py", language = "python", enabled = true }
git-branch = { description = "Show current branch", usage = "git-branch", script = "git_branch.py", language = "python", enabled = true }
git-commit = { description = "Quick commit", usage = "git-commit [message]", script = "git_commit.py", language = "python", enabled = true }
```

```python
# git_status.py
#!/usr/bin/env python3
import subprocess
import sys

def run_git_command(args):
    try:
        result = subprocess.run(['git'] + args, capture_output=True, text=True)
        return result.stdout.strip(), result.stderr.strip(), result.returncode
    except FileNotFoundError:
        return "", "git not found", 1

def main():
    # Get git status
    status, stderr, code = run_git_command(['status', '--porcelain'])
    
    if code != 0:
        print(f"Error: {stderr}")
        return
    
    if not status:
        print("Working directory clean")
        return
    
    print("Git Status:")
    for line in status.split('\n'):
        if line:
            status_code = line[:2]
            file_path = line[3:]
            
            if status_code == 'M ':
                print(f"  Modified: {file_path}")
            elif status_code == ' M':
                print(f"  Modified (staged): {file_path}")
            elif status_code == 'A ':
                print(f"  Added: {file_path}")
            elif status_code == '??':
                print(f"  Untracked: {file_path}")
            else:
                print(f"  {status_code}: {file_path}")

if __name__ == "__main__":
    main()
```

## Best Practices

### 1. Error Handling

Always handle errors gracefully:

```python
#!/usr/bin/env python3
import sys

def main():
    try:
        # Your plugin logic here
        result = process_command(sys.argv[1:])
        print(result)
    except IndexError:
        print("Error: Missing required argument")
        print("Usage: my-command [argument]")
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
```

### 2. Configuration Validation

Validate plugin configuration:

```python
def validate_config(config):
    required_keys = ['api_key', 'endpoint']
    missing_keys = [key for key in required_keys if key not in config]
    
    if missing_keys:
        print(f"Error: Missing required configuration: {', '.join(missing_keys)}")
        print("Run: config set my-plugin.api_key YOUR_API_KEY")
        return False
    
    return True
```

### 3. Documentation

Always include documentation:

```markdown
# My Plugin

## Description
Brief description of what the plugin does.

## Installation
```bash
install-plugin https://github.com/username/my-plugin
```

## Configuration
```bash
config set my-plugin.api_key YOUR_API_KEY
```

## Usage
```bash
my-command [options]
```

## Examples
```bash
my-command --help
my-command process-file.txt
```
```

### 4. Testing

Test your plugins thoroughly:

```bash
# Test command execution
./hello.py "Test User"

# Test with different arguments
./calculator.py "2 + 2"
./calculator.py "invalid expression"

# Test configuration
config set my-plugin.greeting "Hi"
./hello.py "User"
```

## Publishing Plugins

### Create a Repository

1. Create a Git repository for your plugin
2. Include all necessary files (plugin.toml, scripts, README.md)
3. Add proper documentation and examples

### Share Your Plugin

```bash
# Users can install from your repository
install-plugin https://github.com/username/my-plugin

# Or publish as an npm package
npm publish @username/macbrew-plugin-name
```

## Troubleshooting

### Common Issues

**Plugin not loading:**
```bash
# Check plugin syntax
plugins list

# Check plugin logs
tail -f ~/.config/terminal-emulator/plugin.log

# Reload plugins
plugins reload
```

**Command not found:**
```bash
# Verify plugin is enabled
plugins list

# Check command script exists
ls ~/.config/terminal-emulator/plugins/my-plugin/

# Check script permissions
chmod +x ~/.config/terminal-emulator/plugins/my-plugin/command.py
```

**Configuration issues:**
```bash
# Check plugin configuration
config list my-plugin

# Reset plugin configuration
config reset my-plugin
```

## Next Steps

- [Command Reference](/docs/commands) - Browse available commands
- [Configuration](/docs/configuration) - Customize your experience
- [API Reference](/docs/api) - Programmatic access to Macbrew
- [Examples](/docs/examples) - More plugin examples 
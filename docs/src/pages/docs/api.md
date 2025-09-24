---
layout: ../../layouts/Layout.astro
title: API Reference - Macbrew Documentation
description: Programmatic access to Macbrew functionality
---

# API Reference

Macbrew provides a comprehensive API for programmatic access to its functionality.

## Overview

The Macbrew API allows you to:

- Execute commands programmatically
- Access configuration and settings
- Manage plugins
- Interact with the terminal interface
- Extend functionality through hooks

## Core API

### Command Execution

Execute commands programmatically:

```python
import subprocess
import json

def execute_command(command, args=None):
    """Execute a Macbrew command"""
    if args is None:
        args = []
    
    cmd = ['macbrew', command] + args
    result = subprocess.run(cmd, capture_output=True, text=True)
    
    return {
        'stdout': result.stdout,
        'stderr': result.stderr,
        'returncode': result.returncode
    }

# Example usage
result = execute_command('ls', ['-la'])
print(result['stdout'])
```

### Configuration API

Access and modify Macbrew configuration:

```python
import json
import os

class MacbrewConfig:
    def __init__(self):
        self.config_path = os.path.expanduser("~/.config/terminal-emulator/config.json")
    
    def get_config(self):
        """Get current configuration"""
        if os.path.exists(self.config_path):
            with open(self.config_path, 'r') as f:
                return json.load(f)
        return {}
    
    def set_config(self, key, value):
        """Set configuration value"""
        config = self.get_config()
        config[key] = value
        
        os.makedirs(os.path.dirname(self.config_path), exist_ok=True)
        with open(self.config_path, 'w') as f:
            json.dump(config, f, indent=2)
    
    def get_setting(self, key, default=None):
        """Get specific setting"""
        config = self.get_config()
        return config.get(key, default)

# Example usage
config = MacbrewConfig()
theme = config.get_setting('theme', 'default')
config.set_config('theme', 'dark')
```

### Plugin API

Manage plugins programmatically:

```python
import os
import json
import subprocess

class PluginManager:
    def __init__(self):
        self.plugins_dir = os.path.expanduser("~/.config/terminal-emulator/plugins")
    
    def list_plugins(self):
        """List installed plugins"""
        plugins = []
        
        if os.path.exists(self.plugins_dir):
            for plugin_name in os.listdir(self.plugins_dir):
                plugin_path = os.path.join(self.plugins_dir, plugin_name)
                if os.path.isdir(plugin_path):
                    manifest_path = os.path.join(plugin_path, 'plugin.toml')
                    if os.path.exists(manifest_path):
                        plugins.append({
                            'name': plugin_name,
                            'path': plugin_path,
                            'manifest': manifest_path
                        })
        
        return plugins
    
    def install_plugin(self, source):
        """Install a plugin"""
        cmd = ['macbrew', 'install-plugin', source]
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result.returncode == 0
    
    def uninstall_plugin(self, plugin_name):
        """Uninstall a plugin"""
        cmd = ['macbrew', 'uninstall-plugin', plugin_name]
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result.returncode == 0
    
    def enable_plugin(self, plugin_name):
        """Enable a plugin"""
        cmd = ['macbrew', 'enable-plugin', plugin_name]
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result.returncode == 0
    
    def disable_plugin(self, plugin_name):
        """Disable a plugin"""
        cmd = ['macbrew', 'disable-plugin', plugin_name]
        result = subprocess.run(cmd, capture_output=True, text=True)
        return result.returncode == 0

# Example usage
pm = PluginManager()
plugins = pm.list_plugins()
for plugin in plugins:
    print(f"Plugin: {plugin['name']}")
```

## Plugin Development API

### Plugin Context

Plugins have access to a context object with useful information:

```python
import os
import json
import sys

class PluginContext:
    def __init__(self, plugin_name):
        self.plugin_name = plugin_name
        self.plugin_dir = os.path.expanduser(f"~/.config/terminal-emulator/plugins/{plugin_name}")
        self.config_file = os.path.join(self.plugin_dir, 'config.json')
    
    def get_config(self):
        """Get plugin configuration"""
        if os.path.exists(self.config_file):
            with open(self.config_file, 'r') as f:
                return json.load(f)
        return {}
    
    def set_config(self, key, value):
        """Set plugin configuration"""
        config = self.get_config()
        config[key] = value
        
        os.makedirs(os.path.dirname(self.config_file), exist_ok=True)
        with open(self.config_file, 'w') as f:
            json.dump(config, f, indent=2)
    
    def get_macbrew_config(self):
        """Get main Macbrew configuration"""
        config_path = os.path.expanduser("~/.config/terminal-emulator/config.json")
        if os.path.exists(config_path):
            with open(config_path, 'r') as f:
                return json.load(f)
        return {}
    
    def log(self, message, level='info'):
        """Log a message"""
        log_file = os.path.expanduser("~/.config/terminal-emulator/plugin.log")
        timestamp = datetime.now().isoformat()
        
        with open(log_file, 'a') as f:
            f.write(f"[{timestamp}] [{level.upper()}] {self.plugin_name}: {message}\n")

# Example usage in plugin
def main():
    context = PluginContext('my-plugin')
    config = context.get_config()
    
    # Use configuration
    api_key = config.get('api_key', '')
    if not api_key:
        context.log("API key not configured", 'warning')
        print("Error: API key not configured")
        return
    
    # Your plugin logic here
    context.log("Plugin executed successfully")

if __name__ == "__main__":
    main()
```

### Plugin Hooks

Plugins can register hooks for various events:

```python
import os
import json

class PluginHooks:
    def __init__(self, plugin_name):
        self.plugin_name = plugin_name
        self.hooks_dir = os.path.expanduser(f"~/.config/terminal-emulator/plugins/{plugin_name}/hooks")
    
    def register_hook(self, event, script_path):
        """Register a hook for an event"""
        hook_file = os.path.join(self.hooks_dir, f"{event}.py")
        
        os.makedirs(self.hooks_dir, exist_ok=True)
        
        # Create hook script if it doesn't exist
        if not os.path.exists(hook_file):
            with open(hook_file, 'w') as f:
                f.write(f'''#!/usr/bin/env python3
import sys
import os

# Add plugin directory to path
plugin_dir = os.path.dirname(os.path.dirname(__file__))
sys.path.insert(0, plugin_dir)

# Import your hook function
from {script_path} import hook_function

if __name__ == "__main__":
    hook_function()
''')
        
        # Make executable
        os.chmod(hook_file, 0o755)
    
    def execute_hook(self, event, *args):
        """Execute a hook"""
        hook_file = os.path.join(self.hooks_dir, f"{event}.py")
        
        if os.path.exists(hook_file):
            import subprocess
            result = subprocess.run([hook_file] + list(args), capture_output=True, text=True)
            return result.stdout, result.stderr, result.returncode
        
        return "", "", 0

# Example hook implementation
def startup_hook():
    """Hook executed when Macbrew starts"""
    print("My plugin is starting up!")
    # Initialize resources, load data, etc.

def pre_command_hook(command, args):
    """Hook executed before each command"""
    print(f"About to execute: {command} {' '.join(args)}")
    # Validate command, check permissions, etc.

def post_command_hook(command, args, result):
    """Hook executed after each command"""
    print(f"Command completed: {command} (exit code: {result})")
    # Log command, update statistics, etc.
```

## Terminal Interface API

### Terminal Output

Control terminal output and formatting:

```python
import sys
import os

class TerminalOutput:
    # ANSI color codes
    COLORS = {
        'black': '\033[30m',
        'red': '\033[31m',
        'green': '\033[32m',
        'yellow': '\033[33m',
        'blue': '\033[34m',
        'magenta': '\033[35m',
        'cyan': '\033[36m',
        'white': '\033[37m',
        'reset': '\033[0m',
        'bold': '\033[1m',
        'underline': '\033[4m'
    }
    
    @classmethod
    def colorize(cls, text, color='white', bold=False, underline=False):
        """Add color and formatting to text"""
        codes = [cls.COLORS[color]]
        
        if bold:
            codes.append(cls.COLORS['bold'])
        if underline:
            codes.append(cls.COLORS['underline'])
        
        return ''.join(codes) + text + cls.COLORS['reset']
    
    @classmethod
    def print_success(cls, message):
        """Print success message"""
        print(cls.colorize(f"✓ {message}", 'green', bold=True))
    
    @classmethod
    def print_error(cls, message):
        """Print error message"""
        print(cls.colorize(f"✗ {message}", 'red', bold=True))
    
    @classmethod
    def print_warning(cls, message):
        """Print warning message"""
        print(cls.colorize(f"⚠ {message}", 'yellow', bold=True))
    
    @classmethod
    def print_info(cls, message):
        """Print info message"""
        print(cls.colorize(f"ℹ {message}", 'blue'))
    
    @classmethod
    def progress_bar(cls, current, total, width=50):
        """Display a progress bar"""
        progress = int(width * current / total)
        bar = '█' * progress + '░' * (width - progress)
        percentage = int(100 * current / total)
        
        sys.stdout.write(f'\r[{bar}] {percentage}%')
        sys.stdout.flush()
        
        if current == total:
            print()

# Example usage
TerminalOutput.print_success("Operation completed successfully!")
TerminalOutput.print_error("Something went wrong!")
TerminalOutput.print_warning("Please check your configuration")
TerminalOutput.print_info("Processing data...")

# Progress bar example
for i in range(101):
    TerminalOutput.progress_bar(i, 100)
    import time
    time.sleep(0.1)
```

### Input Handling

Handle user input and prompts:

```python
import sys
import getpass

class TerminalInput:
    @staticmethod
    def prompt(message, default=None, required=False):
        """Prompt for user input"""
        if default:
            message = f"{message} [{default}]: "
        else:
            message = f"{message}: "
        
        while True:
            try:
                value = input(message).strip()
                
                if not value and default:
                    value = default
                
                if required and not value:
                    print("This field is required.")
                    continue
                
                return value
            except KeyboardInterrupt:
                print("\nOperation cancelled.")
                sys.exit(1)
    
    @staticmethod
    def password_prompt(message="Enter password"):
        """Prompt for password (hidden input)"""
        return getpass.getpass(f"{message}: ")
    
    @staticmethod
    def confirm(message="Are you sure?", default=False):
        """Confirm an action"""
        default_text = "Y/n" if default else "y/N"
        message = f"{message} [{default_text}]: "
        
        while True:
            response = input(message).strip().lower()
            
            if not response:
                return default
            
            if response in ['y', 'yes']:
                return True
            elif response in ['n', 'no']:
                return False
            else:
                print("Please enter 'y' or 'n'")

# Example usage
name = TerminalInput.prompt("Enter your name", default="Anonymous")
password = TerminalInput.password_prompt("Enter your password")
confirm_delete = TerminalInput.confirm("Delete this file?", default=False)
```

## File System API

### File Operations

Safe file operations with error handling:

```python
import os
import shutil
import json
from pathlib import Path

class FileSystem:
    @staticmethod
    def safe_read(file_path, default=None):
        """Safely read a file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                return f.read()
        except (FileNotFoundError, PermissionError, UnicodeDecodeError):
            return default
    
    @staticmethod
    def safe_write(file_path, content, create_dirs=True):
        """Safely write to a file"""
        try:
            if create_dirs:
                os.makedirs(os.path.dirname(file_path), exist_ok=True)
            
            with open(file_path, 'w', encoding='utf-8') as f:
                f.write(content)
            return True
        except (PermissionError, OSError):
            return False
    
    @staticmethod
    def safe_json_read(file_path, default=None):
        """Safely read JSON file"""
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                return json.load(f)
        except (FileNotFoundError, json.JSONDecodeError):
            return default or {}
    
    @staticmethod
    def safe_json_write(file_path, data, create_dirs=True):
        """Safely write JSON file"""
        try:
            if create_dirs:
                os.makedirs(os.path.dirname(file_path), exist_ok=True)
            
            with open(file_path, 'w', encoding='utf-8') as f:
                json.dump(data, f, indent=2, ensure_ascii=False)
            return True
        except (PermissionError, OSError):
            return False
    
    @staticmethod
    def find_files(pattern, start_path="."):
        """Find files matching pattern"""
        matches = []
        start_path = Path(start_path)
        
        for file_path in start_path.rglob(pattern):
            if file_path.is_file():
                matches.append(str(file_path))
        
        return matches
    
    @staticmethod
    def backup_file(file_path, suffix=".backup"):
        """Create a backup of a file"""
        if not os.path.exists(file_path):
            return False
        
        backup_path = file_path + suffix
        try:
            shutil.copy2(file_path, backup_path)
            return True
        except (PermissionError, OSError):
            return False

# Example usage
content = FileSystem.safe_read("/path/to/file.txt", "Default content")
success = FileSystem.safe_write("/path/to/output.txt", "Hello, World!")

config = FileSystem.safe_json_read("/path/to/config.json")
FileSystem.safe_json_write("/path/to/config.json", {"key": "value"})

files = FileSystem.find_files("*.py", "/path/to/project")
FileSystem.backup_file("/path/to/important.txt")
```

## Network API

### HTTP Requests

Simple HTTP client for plugins:

```python
import requests
import json
from urllib.parse import urljoin

class HTTPClient:
    def __init__(self, base_url=None, timeout=30):
        self.base_url = base_url
        self.timeout = timeout
        self.session = requests.Session()
    
    def get(self, url, params=None, headers=None):
        """Make GET request"""
        if self.base_url:
            url = urljoin(self.base_url, url)
        
        try:
            response = self.session.get(url, params=params, headers=headers, timeout=self.timeout)
            response.raise_for_status()
            return response
        except requests.RequestException as e:
            raise HTTPError(f"GET request failed: {e}")
    
    def post(self, url, data=None, json_data=None, headers=None):
        """Make POST request"""
        if self.base_url:
            url = urljoin(self.base_url, url)
        
        try:
            response = self.session.post(url, data=data, json=json_data, headers=headers, timeout=self.timeout)
            response.raise_for_status()
            return response
        except requests.RequestException as e:
            raise HTTPError(f"POST request failed: {e}")
    
    def download_file(self, url, file_path):
        """Download a file"""
        if self.base_url:
            url = urljoin(self.base_url, url)
        
        try:
            response = self.session.get(url, stream=True, timeout=self.timeout)
            response.raise_for_status()
            
            with open(file_path, 'wb') as f:
                for chunk in response.iter_content(chunk_size=8192):
                    f.write(chunk)
            
            return True
        except requests.RequestException as e:
            raise HTTPError(f"Download failed: {e}")

class HTTPError(Exception):
    pass

# Example usage
client = HTTPClient("https://api.example.com")

try:
    # GET request
    response = client.get("/users", params={"page": 1})
    users = response.json()
    
    # POST request
    data = {"name": "John", "email": "john@example.com"}
    response = client.post("/users", json_data=data)
    new_user = response.json()
    
    # Download file
    client.download_file("/files/document.pdf", "document.pdf")
    
except HTTPError as e:
    print(f"HTTP Error: {e}")
```

## Utility Functions

### Common Utilities

```python
import hashlib
import base64
import uuid
import time
from datetime import datetime

class Utilities:
    @staticmethod
    def generate_id():
        """Generate a unique ID"""
        return str(uuid.uuid4())
    
    @staticmethod
    def hash_string(text, algorithm='sha256'):
        """Hash a string"""
        hash_func = getattr(hashlib, algorithm)()
        hash_func.update(text.encode('utf-8'))
        return hash_func.hexdigest()
    
    @staticmethod
    def encode_base64(data):
        """Encode data to base64"""
        if isinstance(data, str):
            data = data.encode('utf-8')
        return base64.b64encode(data).decode('utf-8')
    
    @staticmethod
    def decode_base64(data):
        """Decode base64 data"""
        return base64.b64decode(data).decode('utf-8')
    
    @staticmethod
    def format_bytes(bytes_value):
        """Format bytes to human readable format"""
        for unit in ['B', 'KB', 'MB', 'GB', 'TB']:
            if bytes_value < 1024.0:
                return f"{bytes_value:.1f} {unit}"
            bytes_value /= 1024.0
        return f"{bytes_value:.1f} PB"
    
    @staticmethod
    def format_duration(seconds):
        """Format duration in seconds to human readable format"""
        if seconds < 60:
            return f"{seconds:.1f}s"
        elif seconds < 3600:
            minutes = seconds / 60
            return f"{minutes:.1f}m"
        elif seconds < 86400:
            hours = seconds / 3600
            return f"{hours:.1f}h"
        else:
            days = seconds / 86400
            return f"{days:.1f}d"
    
    @staticmethod
    def retry(func, max_attempts=3, delay=1):
        """Retry a function with exponential backoff"""
        for attempt in range(max_attempts):
            try:
                return func()
            except Exception as e:
                if attempt == max_attempts - 1:
                    raise e
                
                time.sleep(delay * (2 ** attempt))
    
    @staticmethod
    def validate_email(email):
        """Validate email address format"""
        import re
        pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
        return re.match(pattern, email) is not None

# Example usage
unique_id = Utilities.generate_id()
file_hash = Utilities.hash_string("file content", 'md5')
encoded = Utilities.encode_base64("Hello, World!")
decoded = Utilities.decode_base64(encoded)

size = Utilities.format_bytes(1024 * 1024)  # "1.0 MB"
duration = Utilities.format_duration(3661)   # "1.0h"

# Retry example
def unreliable_function():
    import random
    if random.random() < 0.7:
        raise Exception("Random failure")
    return "Success!"

result = Utilities.retry(unreliable_function, max_attempts=5)
```

## Error Handling

### Exception Classes

```python
class MacbrewError(Exception):
    """Base exception for Macbrew API"""
    pass

class ConfigurationError(MacbrewError):
    """Configuration related errors"""
    pass

class PluginError(MacbrewError):
    """Plugin related errors"""
    pass

class CommandError(MacbrewError):
    """Command execution errors"""
    pass

class ValidationError(MacbrewError):
    """Validation errors"""
    pass

# Example usage
def safe_operation():
    try:
        # Your API calls here
        result = execute_command('ls', ['-la'])
        if result['returncode'] != 0:
            raise CommandError(f"Command failed: {result['stderr']}")
        
        return result['stdout']
    
    except FileNotFoundError:
        raise ConfigurationError("Macbrew not found in PATH")
    except PermissionError:
        raise PluginError("Insufficient permissions")
    except Exception as e:
        raise MacbrewError(f"Unexpected error: {e}")
```

## Best Practices

### 1. Error Handling

Always handle errors gracefully:

```python
def robust_plugin_function():
    try:
        # Your plugin logic
        result = process_data()
        return result
    except FileNotFoundError:
        TerminalOutput.print_error("Required file not found")
        return None
    except PermissionError:
        TerminalOutput.print_error("Insufficient permissions")
        return None
    except Exception as e:
        TerminalOutput.print_error(f"Unexpected error: {e}")
        return None
```

### 2. Configuration Management

Use the configuration API properly:

```python
def get_plugin_config(plugin_name):
    config = MacbrewConfig()
    plugin_config = config.get_setting(f'plugins.{plugin_name}', {})
    
    # Set defaults
    defaults = {
        'api_key': '',
        'timeout': 30,
        'debug': False
    }
    
    return {**defaults, **plugin_config}
```

### 3. Logging

Use the logging system:

```python
def log_plugin_activity(plugin_name, message, level='info'):
    context = PluginContext(plugin_name)
    context.log(message, level)
```

### 4. Input Validation

Validate all inputs:

```python
def validate_input(data, required_fields):
    missing_fields = [field for field in required_fields if field not in data]
    
    if missing_fields:
        raise ValidationError(f"Missing required fields: {', '.join(missing_fields)}")
    
    return True
```

## Next Steps

- [Plugin System](/docs/plugins) - Create custom plugins
- [Configuration](/docs/configuration) - Manage settings
- [Command Reference](/docs/commands) - Available commands
- [Examples](/docs/examples) - API usage examples 
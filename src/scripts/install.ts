#!/usr/bin/env ts-node

import { spawn, execSync } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';
import * as os from 'os';

// ANSI color codes for beautiful output
const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
  white: '\x1b[37m'
};

function colorize(text: string, color: keyof typeof colors): string {
  return `${colors[color]}${text}${colors.reset}`;
}

function log(message: string, color: keyof typeof colors = 'white'): void {
  console.log(colorize(message, color));
}

function logError(message: string): void {
  console.error(colorize(`❌ ${message}`, 'red'));
}

function logSuccess(message: string): void {
  console.log(colorize(`✅ ${message}`, 'green'));
}

function logInfo(message: string): void {
  console.log(colorize(`ℹ️  ${message}`, 'blue'));
}

function logWarning(message: string): void {
  console.log(colorize(`⚠️  ${message}`, 'yellow'));
}

function checkCommand(command: string): boolean {
  try {
    if (process.platform === 'win32') {
      execSync(`where ${command}`, { stdio: 'ignore' });
    } else {
      execSync(`which ${command}`, { stdio: 'ignore' });
    }
    return true;
  } catch {
    return false;
  }
}

function checkRust(): boolean {
  if (!checkCommand('cargo')) {
    logError('Rust is not installed!');
    log('Please install Rust first:', 'yellow');
    log('  curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh', 'green');
    log('  Then restart your terminal and run: npm install', 'green');
    return false;
  }
  return true;
}

function checkPython(): boolean {
  if (!checkCommand('python3') && !checkCommand('python')) {
    logError('Python 3 is not installed!');
    log('Please install Python 3.8+ first:', 'yellow');
    log('  Visit: https://www.python.org/downloads/', 'green');
    return false;
  }
  return true;
}

function checkNode(): boolean {
  if (!checkCommand('node')) {
    logError('Node.js is not installed!');
    log('Please install Node.js 14+ first:', 'yellow');
    log('  Visit: https://nodejs.org/', 'green');
    return false;
  }
  return true;
}

function buildRustBinary(): boolean {
  log('🔨 Building Rust binary...', 'cyan');
  
  try {
    // Run cargo build in release mode
    execSync('cargo build --release', { 
      stdio: 'inherit',
      cwd: process.cwd()
    });
    
    logSuccess('Rust binary built successfully!');
    return true;
  } catch (error) {
    logError('Failed to build Rust binary');
    logError((error as Error).message);
    return false;
  }
}

function buildTypeScript(): boolean {
  log('🔨 Building TypeScript...', 'cyan');
  
  try {
    // Run TypeScript compilation
    execSync('npm run build', { 
      stdio: 'inherit',
      cwd: process.cwd()
    });
    
    logSuccess('TypeScript compiled successfully!');
    return true;
  } catch (error) {
    logError('Failed to compile TypeScript');
    logError((error as Error).message);
    return false;
  }
}

function setupConfiguration(): boolean {
  log('⚙️  Setting up configuration...', 'cyan');
  
  const configDir: string = path.join(os.homedir(), '.config', 'terminal-emulator');
  const configFile: string = path.join(configDir, 'config.json');
  
  try {
    // Create config directory if it doesn't exist
    if (!fs.existsSync(configDir)) {
      fs.mkdirSync(configDir, { recursive: true });
    }
    
    // Create default config if it doesn't exist
    if (!fs.existsSync(configFile)) {
      const defaultConfig = {
        prompt_style: {
          show_hostname: true,
          show_username: true,
          show_path: true,
          show_git_branch: false,
          format: "{username}@{hostname}:{path} $ "
        },
        history_size: 1000,
        auto_completion: true,
        syntax_highlighting: true,
        colors: {
          prompt: "\u001b[32m",
          command: "\u001b[36m",
          output: "\u001b[0m",
          error: "\u001b[31m",
          success: "\u001b[32m",
          warning: "\u001b[33m"
        }
      };
      
      fs.writeFileSync(configFile, JSON.stringify(defaultConfig, null, 2));
      logSuccess('Configuration file created');
    } else {
      logInfo('Configuration file already exists');
    }
    
    return true;
  } catch (error) {
    logError('Failed to setup configuration');
    logError((error as Error).message);
    return false;
  }
}

function setupPlugins(): boolean {
  log('🔌 Setting up example plugins...', 'cyan');
  
  const pluginsDir: string = path.join(os.homedir(), '.config', 'terminal-emulator', 'plugins');
  const examplePluginDir: string = path.join(pluginsDir, 'example');
  
  try {
    // Create plugins directory if it doesn't exist
    if (!fs.existsSync(pluginsDir)) {
      fs.mkdirSync(pluginsDir, { recursive: true });
    }
    
    // Create example plugin if it doesn't exist
    if (!fs.existsSync(examplePluginDir)) {
      fs.mkdirSync(examplePluginDir, { recursive: true });
      
      // Create plugin.toml
      const pluginToml = `name = "example"
version = "1.0.0"
description = "Example plugin for Macbrew"
author = "Macbrew Team"
commands = [
    { name = "hello", description = "Say hello", usage = "hello [name]", script = "hello.py", language = "python", enabled = true },
    { name = "weather", description = "Get weather info", usage = "weather [city]", script = "weather.py", language = "python", enabled = true }
]`;
      
      fs.writeFileSync(path.join(examplePluginDir, 'plugin.toml'), pluginToml);
      
      // Create hello.py
      const helloPy = `#!/usr/bin/env python3
import sys

def main():
    if len(sys.argv) > 1:
        name = sys.argv[1]
        print(f"Hello, {name}!")
    else:
        print("Hello, World!")

if __name__ == "__main__":
    main()`;
      
      fs.writeFileSync(path.join(examplePluginDir, 'hello.py'), helloPy);
      
      // Create weather.py
      const weatherPy = `#!/usr/bin/env python3
import sys
import random

def main():
    cities = ["New York", "London", "Tokyo", "Paris", "Sydney", "Berlin", "Moscow", "Beijing"]
    weather_types = ["sunny", "cloudy", "rainy", "snowy", "windy", "foggy"]
    
    if len(sys.argv) > 1:
        city = sys.argv[1]
    else:
        city = random.choice(cities)
    
    weather = random.choice(weather_types)
    temp = random.randint(-10, 35)
    
    print(f"Weather in {city}: {weather}, {temp}°C")

if __name__ == "__main__":
    main()`;
      
      fs.writeFileSync(path.join(examplePluginDir, 'weather.py'), weatherPy);
      
      logSuccess('Example plugins created');
    } else {
      logInfo('Example plugins already exist');
    }
    
    return true;
  } catch (error) {
    logError('Failed to setup plugins');
    logError((error as Error).message);
    return false;
  }
}

function main(): void {
  log('🚀 Installing Macbrew...', 'cyan');
  log('Advanced terminal emulator built with Rust and Python', 'blue');
  log('');
  
  // Check prerequisites
  log('🔍 Checking prerequisites...', 'cyan');
  
  if (!checkRust()) {
    process.exit(1);
  }
  
  if (!checkPython()) {
    process.exit(1);
  }
  
  if (!checkNode()) {
    process.exit(1);
  }
  
  logSuccess('All prerequisites are satisfied!');
  log('');
  
  // Build Rust binary
  if (!buildRustBinary()) {
    process.exit(1);
  }
  
  // Build TypeScript
  if (!buildTypeScript()) {
    process.exit(1);
  }
  
  log('');
  
  // Setup configuration
  if (!setupConfiguration()) {
    logWarning('Configuration setup failed, but continuing...');
  }
  
  // Setup plugins
  if (!setupPlugins()) {
    logWarning('Plugin setup failed, but continuing...');
  }
  
  log('');
  logSuccess('Macbrew installation completed successfully!');
  log('');
  log('🎉 You can now run Macbrew using:', 'cyan');
  log('   npx macbrew', 'green');
  log('   or', 'white');
  log('   npm install -g macbrew && macbrew', 'green');
  log('');
  log('📖 For more information, visit:', 'cyan');
  log('   https://github.com/Zemerik/Macbrew', 'blue');
  log('');
  log('Happy coding! 🚀', 'magenta');
}

// Run the main function
main(); 
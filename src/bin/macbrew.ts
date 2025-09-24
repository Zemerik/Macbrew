#!/usr/bin/env ts-node

import { spawn, ChildProcess } from 'child_process';
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

function getExecutablePath(): string {
  const platform: string = os.platform();
  const arch: string = os.arch();
  
  // Get the directory where this script is located
  const scriptDir: string = __dirname;
  const packageDir: string = path.dirname(path.dirname(scriptDir)); // Go up two levels from dist/bin to package root
  
  let executableName: string;
  if (platform === 'win32') {
    executableName = 'terminal-emulator.exe';
  } else {
    executableName = 'terminal-emulator';
  }
  
  const executablePath: string = path.join(packageDir, 'target', 'release', executableName);
  
  // Check if the executable exists
  if (!fs.existsSync(executablePath)) {
    console.error(colorize('❌ Error: Macbrew executable not found!', 'red'));
    console.error(colorize(`Expected path: ${executablePath}`, 'yellow'));
    console.error(colorize('\nThis usually means the Rust binary hasn\'t been built yet.', 'yellow'));
    console.error(colorize('Please run: npm install', 'cyan'));
    process.exit(1);
  }
  
  return executablePath;
}

function showWelcomeMessage(): void {
  console.log(colorize('\n🚀 Welcome to Macbrew!', 'cyan'));
  console.log(colorize('Advanced terminal emulator built with Rust and Python', 'blue'));
  console.log(colorize('Type "help" for available commands, "exit" to quit\n', 'green'));
}

function showErrorAndHelp(error: Error): void {
  console.error(colorize('\n❌ Error launching Macbrew:', 'red'));
  console.error(colorize(error.message, 'yellow'));
  
  console.log(colorize('\n🔧 Troubleshooting:', 'cyan'));
  console.log(colorize('1. Make sure you have Rust installed:', 'white'));
  console.log(colorize('   curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh', 'green'));
  console.log(colorize('2. Reinstall the package:', 'white'));
  console.log(colorize('   npm uninstall -g macbrew && npm install -g macbrew', 'green'));
  console.log(colorize('3. Check system requirements:', 'white'));
  console.log(colorize('   - Rust 1.70+', 'green'));
  console.log(colorize('   - Python 3.8+', 'green'));
  console.log(colorize('   - Node.js 14+', 'green'));
  
  console.log(colorize('\n📖 For more help, visit:', 'cyan'));
  console.log(colorize('   https://github.com/Zemerik/Macbrew', 'blue'));
}

function main(): void {
  try {
    // Check if this is the first run
    const isFirstRun: boolean = process.argv.includes('--first-run');
    
    if (isFirstRun) {
      showWelcomeMessage();
    }
    
    const executablePath: string = getExecutablePath();
    
    // Spawn the Rust terminal emulator
    const child: ChildProcess = spawn(executablePath, process.argv.slice(2), {
      stdio: 'inherit',
      cwd: process.cwd(),
      env: process.env
    });
    
    // Handle process events
    child.on('error', (error: Error) => {
      showErrorAndHelp(error);
      process.exit(1);
    });
    
    child.on('exit', (code: number | null) => {
      process.exit(code || 0);
    });
    
    // Handle SIGINT (Ctrl+C)
    process.on('SIGINT', () => {
      child.kill('SIGINT');
    });
    
    // Handle SIGTERM
    process.on('SIGTERM', () => {
      child.kill('SIGTERM');
    });
    
  } catch (error) {
    showErrorAndHelp(error as Error);
    process.exit(1);
  }
}

// Run the main function
main(); 
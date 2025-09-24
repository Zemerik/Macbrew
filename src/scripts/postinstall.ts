#!/usr/bin/env ts-node

import { execSync } from 'child_process';
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

function logSuccess(message: string): void {
  console.log(colorize(`✅ ${message}`, 'green'));
}

function logInfo(message: string): void {
  console.log(colorize(`ℹ️  ${message}`, 'blue'));
}

function logWarning(message: string): void {
  console.log(colorize(`⚠️  ${message}`, 'yellow'));
}

function checkFirstRun(): boolean {
  const configDir: string = path.join(os.homedir(), '.config', 'terminal-emulator');
  const firstRunFile: string = path.join(configDir, '.first-run');
  
  if (!fs.existsSync(firstRunFile)) {
    return true;
  }
  return false;
}

function markFirstRun(): void {
  const configDir: string = path.join(os.homedir(), '.config', 'terminal-emulator');
  const firstRunFile: string = path.join(configDir, '.first-run');
  
  try {
    if (!fs.existsSync(configDir)) {
      fs.mkdirSync(configDir, { recursive: true });
    }
    fs.writeFileSync(firstRunFile, new Date().toISOString());
  } catch (error) {
    // Silently fail - not critical
  }
}

function showWelcomeMessage(): void {
  console.log('');
  console.log(colorize('🎉 Welcome to Macbrew!', 'cyan'));
  console.log(colorize('Advanced terminal emulator built with Rust and Python', 'blue'));
  console.log('');
  console.log(colorize('🚀 Quick Start:', 'cyan'));
  console.log(colorize('   npx macbrew', 'green'));
  console.log(colorize('   or', 'white'));
  console.log(colorize('   macbrew', 'green'));
  console.log('');
  console.log(colorize('📖 Documentation:', 'cyan'));
  console.log(colorize('   https://github.com/Zemerik/Macbrew', 'blue'));
  console.log('');
  console.log(colorize('✨ Enjoy your new terminal experience!', 'magenta'));
  console.log('');
}

function main(): void {
  // Check if this is the first run
  if (checkFirstRun()) {
    showWelcomeMessage();
    markFirstRun();
  } else {
    logInfo('Macbrew post-installation completed');
  }
}

// Run the main function
main(); 
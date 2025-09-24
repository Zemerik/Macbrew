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

function removeConfigFiles(): boolean {
  log('🗑️  Removing configuration files...', 'cyan');
  
  const configDir: string = path.join(os.homedir(), '.config', 'terminal-emulator');
  
  try {
    if (fs.existsSync(configDir)) {
      // Remove the entire config directory
      fs.rmSync(configDir, { recursive: true, force: true });
      logSuccess('Configuration files removed');
    } else {
      logInfo('No configuration files found');
    }
  } catch (error) {
    logError('Failed to remove configuration files');
    logError((error as Error).message);
    return false;
  }
  
  return true;
}

function removeRustBinary(): boolean {
  log('🔨 Removing Rust binary...', 'cyan');
  
  const packageDir: string = path.dirname(path.dirname(__dirname)); // Go up three levels from src/scripts to package root
  const targetDir: string = path.join(packageDir, 'target');
  
  try {
    if (fs.existsSync(targetDir)) {
      // Remove the entire target directory
      fs.rmSync(targetDir, { recursive: true, force: true });
      logSuccess('Rust binary removed');
    } else {
      logInfo('No Rust binary found');
    }
  } catch (error) {
    logError('Failed to remove Rust binary');
    logError((error as Error).message);
    return false;
  }
  
  return true;
}

function removeTypeScriptBuild(): boolean {
  log('🔨 Removing TypeScript build...', 'cyan');
  
  const packageDir: string = path.dirname(path.dirname(__dirname)); // Go up three levels from src/scripts to package root
  const distDir: string = path.join(packageDir, 'dist');
  
  try {
    if (fs.existsSync(distDir)) {
      // Remove the entire dist directory
      fs.rmSync(distDir, { recursive: true, force: true });
      logSuccess('TypeScript build removed');
    } else {
      logInfo('No TypeScript build found');
    }
  } catch (error) {
    logError('Failed to remove TypeScript build');
    logError((error as Error).message);
    return false;
  }
  
  return true;
}

function showUninstallMessage(): void {
  console.log('');
  log('👋 Macbrew has been uninstalled!', 'cyan');
  log('');
  log('📝 What was removed:', 'cyan');
  log('   • Configuration files (~/.config/terminal-emulator/)', 'white');
  log('   • Rust binary (target/release/)', 'white');
  log('   • TypeScript build (dist/)', 'white');
  log('   • Plugin files', 'white');
  log('');
  log('💡 To reinstall Macbrew:', 'cyan');
  log('   npm install -g macbrew', 'green');
  log('   or', 'white');
  log('   npx macbrew', 'green');
  log('');
  log('📖 Visit: https://github.com/Zemerik/Macbrew', 'blue');
  log('');
  log('Thanks for using Macbrew! 🚀', 'magenta');
}

function main(): void {
  log('🚀 Uninstalling Macbrew...', 'cyan');
  log('Advanced terminal emulator built with Rust and Python', 'blue');
  log('');
  
  // Remove configuration files
  if (!removeConfigFiles()) {
    logWarning('Some configuration files could not be removed');
  }
  
  // Remove Rust binary
  if (!removeRustBinary()) {
    logWarning('Rust binary could not be removed');
  }
  
  // Remove TypeScript build
  if (!removeTypeScriptBuild()) {
    logWarning('TypeScript build could not be removed');
  }
  
  log('');
  showUninstallMessage();
}

// Run the main function
main(); 
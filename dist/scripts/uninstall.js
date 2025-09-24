#!/usr/bin/env ts-node
"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
const path = __importStar(require("path"));
const fs = __importStar(require("fs"));
const os = __importStar(require("os"));
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
function colorize(text, color) {
    return `${colors[color]}${text}${colors.reset}`;
}
function log(message, color = 'white') {
    console.log(colorize(message, color));
}
function logError(message) {
    console.error(colorize(`❌ ${message}`, 'red'));
}
function logSuccess(message) {
    console.log(colorize(`✅ ${message}`, 'green'));
}
function logInfo(message) {
    console.log(colorize(`ℹ️  ${message}`, 'blue'));
}
function logWarning(message) {
    console.log(colorize(`⚠️  ${message}`, 'yellow'));
}
function removeConfigFiles() {
    log('🗑️  Removing configuration files...', 'cyan');
    const configDir = path.join(os.homedir(), '.config', 'terminal-emulator');
    try {
        if (fs.existsSync(configDir)) {
            // Remove the entire config directory
            fs.rmSync(configDir, { recursive: true, force: true });
            logSuccess('Configuration files removed');
        }
        else {
            logInfo('No configuration files found');
        }
    }
    catch (error) {
        logError('Failed to remove configuration files');
        logError(error.message);
        return false;
    }
    return true;
}
function removeRustBinary() {
    log('🔨 Removing Rust binary...', 'cyan');
    const packageDir = path.dirname(path.dirname(__dirname)); // Go up three levels from src/scripts to package root
    const targetDir = path.join(packageDir, 'target');
    try {
        if (fs.existsSync(targetDir)) {
            // Remove the entire target directory
            fs.rmSync(targetDir, { recursive: true, force: true });
            logSuccess('Rust binary removed');
        }
        else {
            logInfo('No Rust binary found');
        }
    }
    catch (error) {
        logError('Failed to remove Rust binary');
        logError(error.message);
        return false;
    }
    return true;
}
function removeTypeScriptBuild() {
    log('🔨 Removing TypeScript build...', 'cyan');
    const packageDir = path.dirname(path.dirname(__dirname)); // Go up three levels from src/scripts to package root
    const distDir = path.join(packageDir, 'dist');
    try {
        if (fs.existsSync(distDir)) {
            // Remove the entire dist directory
            fs.rmSync(distDir, { recursive: true, force: true });
            logSuccess('TypeScript build removed');
        }
        else {
            logInfo('No TypeScript build found');
        }
    }
    catch (error) {
        logError('Failed to remove TypeScript build');
        logError(error.message);
        return false;
    }
    return true;
}
function showUninstallMessage() {
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
function main() {
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
//# sourceMappingURL=uninstall.js.map
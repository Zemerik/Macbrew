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
const child_process_1 = require("child_process");
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
function getExecutablePath() {
    const platform = os.platform();
    const arch = os.arch();
    // Get the directory where this script is located
    const scriptDir = __dirname;
    const packageDir = path.dirname(path.dirname(scriptDir)); // Go up two levels from dist/bin to package root
    let executableName;
    if (platform === 'win32') {
        executableName = 'terminal-emulator.exe';
    }
    else {
        executableName = 'terminal-emulator';
    }
    const executablePath = path.join(packageDir, 'target', 'release', executableName);
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
function showWelcomeMessage() {
    console.log(colorize('\n🚀 Welcome to Macbrew!', 'cyan'));
    console.log(colorize('Advanced terminal emulator built with Rust and Python', 'blue'));
    console.log(colorize('Type "help" for available commands, "exit" to quit\n', 'green'));
}
function showErrorAndHelp(error) {
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
function main() {
    try {
        // Check if this is the first run
        const isFirstRun = process.argv.includes('--first-run');
        if (isFirstRun) {
            showWelcomeMessage();
        }
        const executablePath = getExecutablePath();
        // Spawn the Rust terminal emulator
        const child = (0, child_process_1.spawn)(executablePath, process.argv.slice(2), {
            stdio: 'inherit',
            cwd: process.cwd(),
            env: process.env
        });
        // Handle process events
        child.on('error', (error) => {
            showErrorAndHelp(error);
            process.exit(1);
        });
        child.on('exit', (code) => {
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
    }
    catch (error) {
        showErrorAndHelp(error);
        process.exit(1);
    }
}
// Run the main function
main();
//# sourceMappingURL=macbrew.js.map
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
function logSuccess(message) {
    console.log(colorize(`✅ ${message}`, 'green'));
}
function logInfo(message) {
    console.log(colorize(`ℹ️  ${message}`, 'blue'));
}
function logWarning(message) {
    console.log(colorize(`⚠️  ${message}`, 'yellow'));
}
function checkFirstRun() {
    const configDir = path.join(os.homedir(), '.config', 'terminal-emulator');
    const firstRunFile = path.join(configDir, '.first-run');
    if (!fs.existsSync(firstRunFile)) {
        return true;
    }
    return false;
}
function markFirstRun() {
    const configDir = path.join(os.homedir(), '.config', 'terminal-emulator');
    const firstRunFile = path.join(configDir, '.first-run');
    try {
        if (!fs.existsSync(configDir)) {
            fs.mkdirSync(configDir, { recursive: true });
        }
        fs.writeFileSync(firstRunFile, new Date().toISOString());
    }
    catch (error) {
        // Silently fail - not critical
    }
}
function showWelcomeMessage() {
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
function main() {
    // Check if this is the first run
    if (checkFirstRun()) {
        showWelcomeMessage();
        markFirstRun();
    }
    else {
        logInfo('Macbrew post-installation completed');
    }
}
// Run the main function
main();
//# sourceMappingURL=postinstall.js.map
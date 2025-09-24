#!/usr/bin/env node
"use strict";
/**
 * Macbrew - Advanced Terminal Emulator
 *
 * This is the main entry point for the Macbrew package.
 * It exports the main functionality and provides a clean API.
 */
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.NAME = exports.VERSION = exports.ConfigManager = exports.PluginManager = exports.Terminal = exports.Macbrew = void 0;
exports.createMacbrew = createMacbrew;
var macbrew_1 = require("./lib/macbrew");
Object.defineProperty(exports, "Macbrew", { enumerable: true, get: function () { return __importDefault(macbrew_1).default; } });
var terminal_1 = require("./lib/terminal");
Object.defineProperty(exports, "Terminal", { enumerable: true, get: function () { return __importDefault(terminal_1).default; } });
var plugin_manager_1 = require("./lib/plugin-manager");
Object.defineProperty(exports, "PluginManager", { enumerable: true, get: function () { return __importDefault(plugin_manager_1).default; } });
var config_manager_1 = require("./lib/config-manager");
Object.defineProperty(exports, "ConfigManager", { enumerable: true, get: function () { return __importDefault(config_manager_1).default; } });
// Version information
exports.VERSION = '1.0.0';
exports.NAME = 'Macbrew';
// Main function for programmatic usage
function createMacbrew(options) {
    const { Macbrew } = require('./lib/macbrew');
    return new Macbrew(options);
}
// Default export
exports.default = createMacbrew;
//# sourceMappingURL=index.js.map
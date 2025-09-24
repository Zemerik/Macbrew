#!/usr/bin/env node
/**
 * Macbrew - Advanced Terminal Emulator
 *
 * This is the main entry point for the Macbrew package.
 * It exports the main functionality and provides a clean API.
 */
export { default as Macbrew } from './lib/macbrew';
export { default as Terminal } from './lib/terminal';
export { default as PluginManager } from './lib/plugin-manager';
export { default as ConfigManager } from './lib/config-manager';
export type { MacbrewConfig } from './types/config';
export type { Plugin, PluginCommand } from './types/plugin';
export type { TerminalOptions } from './types/terminal';
export declare const VERSION = "1.0.0";
export declare const NAME = "Macbrew";
export declare function createMacbrew(options?: any): any;
export default createMacbrew;
//# sourceMappingURL=index.d.ts.map
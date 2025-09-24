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

// Re-export types
export type { MacbrewConfig } from './types/config';
export type { Plugin, PluginCommand } from './types/plugin';
export type { TerminalOptions } from './types/terminal';

// Version information
export const VERSION = '1.0.0';
export const NAME = 'Macbrew';

// Main function for programmatic usage
export function createMacbrew(options?: any) {
  const { Macbrew } = require('./lib/macbrew');
  return new Macbrew(options);
}

// Default export
export default createMacbrew; 
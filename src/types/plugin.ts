/**
 * Plugin types for Macbrew
 */

export interface PluginCommand {
  name: string;
  description: string;
  usage: string;
  script: string;
  language: 'python' | 'javascript' | 'typescript' | 'bash' | 'shell';
  enabled: boolean;
  args?: string[];
  env?: Record<string, string>;
}

export interface Plugin {
  name: string;
  version: string;
  description: string;
  author: string;
  commands: PluginCommand[];
  dependencies?: string[];
  config?: Record<string, any>;
}

export interface PluginManifest {
  name: string;
  version: string;
  description: string;
  author: string;
  commands: PluginCommand[];
  dependencies?: string[];
  config?: Record<string, any>;
}

export interface PluginOptions {
  pluginDir?: string;
  autoLoad?: boolean;
  enableLogging?: boolean;
} 
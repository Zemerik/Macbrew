/**
 * Configuration types for Macbrew
 */

export interface PromptStyle {
  show_hostname: boolean;
  show_username: boolean;
  show_path: boolean;
  show_git_branch: boolean;
  format: string;
}

export interface Colors {
  prompt: string;
  command: string;
  output: string;
  error: string;
  success: string;
  warning: string;
}

export interface MacbrewConfig {
  prompt_style: PromptStyle;
  history_size: number;
  auto_completion: boolean;
  syntax_highlighting: boolean;
  colors: Colors;
  aliases?: Record<string, string>;
  environment?: Record<string, string>;
}

export interface ConfigOptions {
  configPath?: string;
  defaultConfig?: Partial<MacbrewConfig>;
} 
/**
 * Terminal types for Macbrew
 */

export interface TerminalOptions {
  prompt?: string;
  historySize?: number;
  autoCompletion?: boolean;
  syntaxHighlighting?: boolean;
  colors?: boolean;
  quiet?: boolean;
  debug?: boolean;
}

export interface CommandResult {
  success: boolean;
  output: string;
  error?: string;
  exitCode: number;
  duration: number;
}

export interface HistoryEntry {
  command: string;
  timestamp: Date;
  exitCode: number;
  duration: number;
}

export interface Job {
  id: number;
  command: string;
  pid: number;
  status: 'running' | 'stopped' | 'completed' | 'failed';
  startTime: Date;
  endTime?: Date;
  exitCode?: number;
}

export interface AutocompleteResult {
  suggestions: string[];
  partial: string;
  cursor: number;
} 
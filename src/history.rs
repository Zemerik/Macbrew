use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use anyhow::Result;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub command: String,
    pub timestamp: DateTime<Utc>,
    pub exit_code: i32,
}

pub struct CommandHistory {
    entries: VecDeque<HistoryEntry>,
    max_entries: usize,
    history_file: PathBuf,
}

impl CommandHistory {
    pub fn new() -> Self {
        let mut history = CommandHistory {
            entries: VecDeque::new(),
            max_entries: 1000,
            history_file: Self::get_history_file_path(),
        };
        
        // Load existing history
        if let Err(e) = history.load() {
            eprintln!("Warning: Could not load command history: {}", e);
        }
        
        history
    }

    pub fn add_command(&mut self, command: &str) {
        let entry = HistoryEntry {
            command: command.to_string(),
            timestamp: Utc::now(),
            exit_code: 0, // We'll assume success for now
        };

        self.entries.push_back(entry);

        // Keep only the last max_entries
        while self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }

        // Save history periodically
        if self.entries.len() % 10 == 0 {
            if let Err(e) = self.save() {
                eprintln!("Warning: Could not save command history: {}", e);
            }
        }
    }

    pub fn get_recent(&self, count: usize) -> Vec<&String> {
        let start = if self.entries.len() > count {
            self.entries.len() - count
        } else {
            0
        };

        self.entries
            .iter()
            .skip(start)
            .map(|entry| &entry.command)
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&HistoryEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.command.contains(query))
            .collect()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
        if let Err(e) = self.save() {
            eprintln!("Warning: Could not save empty history: {}", e);
        }
    }

    pub fn get_all(&self) -> &VecDeque<HistoryEntry> {
        &self.entries
    }

    pub fn get_by_index(&self, index: usize) -> Option<&HistoryEntry> {
        self.entries.get(index)
    }

    pub fn get_last(&self) -> Option<&HistoryEntry> {
        self.entries.back()
    }

    pub fn get_last_n(&self, n: usize) -> Vec<&HistoryEntry> {
        let start = if self.entries.len() > n {
            self.entries.len() - n
        } else {
            0
        };

        self.entries
            .iter()
            .skip(start)
            .collect()
    }

    pub fn set_max_entries(&mut self, max: usize) {
        self.max_entries = max;
        while self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }

    pub fn get_max_entries(&self) -> usize {
        self.max_entries
    }

    fn get_history_file_path() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".terminal_emulator_history");
        path
    }

    fn save(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.entries)?;
        fs::write(&self.history_file, json)?;
        Ok(())
    }

    fn load(&mut self) -> Result<()> {
        if !self.history_file.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.history_file)?;
        let entries: Vec<HistoryEntry> = serde_json::from_str(&content)?;
        
        self.entries.clear();
        for entry in entries {
            self.entries.push_back(entry);
        }

        Ok(())
    }
}

impl Drop for CommandHistory {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            eprintln!("Error saving command history: {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_command() {
        let mut history = CommandHistory::new();
        history.add_command("ls -la");
        history.add_command("cd /tmp");
        
        assert_eq!(history.entries.len(), 2);
        assert_eq!(history.entries[0].command, "ls -la");
        assert_eq!(history.entries[1].command, "cd /tmp");
    }

    #[test]
    fn test_get_recent() {
        let mut history = CommandHistory::new();
        history.add_command("command1");
        history.add_command("command2");
        history.add_command("command3");
        
        let recent = history.get_recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0], "command2");
        assert_eq!(recent[1], "command3");
    }

    #[test]
    fn test_search() {
        let mut history = CommandHistory::new();
        history.add_command("ls -la");
        history.add_command("cd /tmp");
        history.add_command("ls -l");
        
        let results = history.search("ls");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_max_entries() {
        let mut history = CommandHistory::new();
        history.set_max_entries(2);
        
        history.add_command("command1");
        history.add_command("command2");
        history.add_command("command3");
        
        assert_eq!(history.entries.len(), 2);
        assert_eq!(history.entries[0].command, "command2");
        assert_eq!(history.entries[1].command, "command3");
    }
} 
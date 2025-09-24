use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use toml;
use colored::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub commands: Vec<PluginCommand>,
    pub dependencies: Option<Vec<String>>,
    pub config: Option<HashMap<String, toml::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginCommand {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub script: String,
    pub language: String, // "python", "bash", "rust", etc.
    pub enabled: bool,
}

#[derive(Debug)]
pub struct Plugin {
    pub manifest: PluginManifest,
    pub path: PathBuf,
    pub loaded: bool,
}

pub struct PluginManager {
    plugins: HashMap<String, Plugin>,
    plugin_dir: PathBuf,
}

impl PluginManager {
    pub fn new() -> Result<Self> {
        let plugin_dir = Self::get_plugin_directory()?;
        fs::create_dir_all(&plugin_dir)?;

        Ok(PluginManager {
            plugins: HashMap::new(),
            plugin_dir,
        })
    }

    pub fn load_plugins(&mut self) -> Result<()> {
        if !self.plugin_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&self.plugin_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                if let Err(e) = self.load_plugin(&path) {
                    eprintln!("Failed to load plugin from {}: {}", path.display(), e);
                }
            }
        }

        Ok(())
    }

    pub fn load_plugin(&mut self, plugin_path: &Path) -> Result<()> {
        let manifest_path = plugin_path.join("plugin.toml");
        if !manifest_path.exists() {
            return Err(anyhow::anyhow!("No plugin.toml found in {}", plugin_path.display()));
        }

        let manifest_content = fs::read_to_string(&manifest_path)?;
        let manifest: PluginManifest = toml::from_str(&manifest_content)?;

        let plugin = Plugin {
            manifest,
            path: plugin_path.to_path_buf(),
            loaded: true,
        };

        let name = plugin.manifest.name.clone();
        self.plugins.insert(name.clone(), plugin);
        println!("Loaded plugin: {}", name.green());

        Ok(())
    }

    pub fn unload_plugin(&mut self, name: &str) -> Result<()> {
        if let Some(mut plugin) = self.plugins.remove(name) {
            plugin.loaded = false;
            println!("Unloaded plugin: {}", name.yellow());
        }

        Ok(())
    }

    pub fn reload_plugin(&mut self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get(name) {
            let path = plugin.path.clone();
            self.unload_plugin(name)?;
            self.load_plugin(&path)?;
        }

        Ok(())
    }

    pub fn list_plugins(&self) -> Vec<&Plugin> {
        self.plugins.values().collect()
    }

    pub fn get_plugin(&self, name: &str) -> Option<&Plugin> {
        self.plugins.get(name)
    }

    pub fn get_plugin_commands(&self) -> Vec<&PluginCommand> {
        let mut commands = Vec::new();
        for plugin in self.plugins.values() {
            if plugin.loaded {
                commands.extend(plugin.manifest.commands.iter());
            }
        }
        commands
    }

    pub fn execute_plugin_command(&self, command_name: &str, args: &[&str]) -> Result<()> {
        for plugin in self.plugins.values() {
            if !plugin.loaded {
                continue;
            }

            for cmd in &plugin.manifest.commands {
                if cmd.name == command_name && cmd.enabled {
                    return self.run_plugin_script(&plugin, cmd, args);
                }
            }
        }

        Err(anyhow::anyhow!("Plugin command '{}' not found", command_name))
    }

    fn run_plugin_script(&self, plugin: &Plugin, command: &PluginCommand, args: &[&str]) -> Result<()> {
        let script_path = plugin.path.join(&command.script);
        
        match command.language.as_str() {
            "python" => {
                let output = std::process::Command::new("python3")
                    .arg(&script_path)
                    .args(args)
                    .output()
                    .context("Failed to execute Python plugin script")?;

                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            "bash" | "sh" => {
                let output = std::process::Command::new("bash")
                    .arg(&script_path)
                    .args(args)
                    .output()
                    .context("Failed to execute bash plugin script")?;

                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            "rust" => {
                // For Rust plugins, we would need to compile them first
                // This is a simplified version
                let output = std::process::Command::new("cargo")
                    .arg("run")
                    .arg("--manifest-path")
                    .arg(&script_path)
                    .args(args)
                    .output()
                    .context("Failed to execute Rust plugin script")?;

                if !output.stdout.is_empty() {
                    print!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprint!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            _ => {
                return Err(anyhow::anyhow!("Unsupported plugin language: {}", command.language));
            }
        }

        Ok(())
    }

    pub fn install_plugin(&mut self, _source: &str) -> Result<()> {
        // This would download and install a plugin from a repository
        // For now, we'll create a simple example plugin
        let plugin_name = "example";
        let plugin_path = self.plugin_dir.join(plugin_name);
        
        if plugin_path.exists() {
            return Err(anyhow::anyhow!("Plugin {} already exists", plugin_name));
        }

        fs::create_dir(&plugin_path)?;

        // Create plugin manifest
        let manifest = PluginManifest {
            name: plugin_name.to_string(),
            version: "1.0.0".to_string(),
            description: "Example plugin".to_string(),
            author: "Terminal Emulator".to_string(),
            commands: vec![
                PluginCommand {
                    name: "hello".to_string(),
                    description: "Say hello".to_string(),
                    usage: "hello [name]".to_string(),
                    script: "hello.py".to_string(),
                    language: "python".to_string(),
                    enabled: true,
                }
            ],
            dependencies: None,
            config: None,
        };

        let manifest_content = toml::to_string_pretty(&manifest)?;
        fs::write(plugin_path.join("plugin.toml"), manifest_content)?;

        // Create example script
        let script_content = r#"#!/usr/bin/env python3
import sys

def main():
    if len(sys.argv) > 1:
        name = sys.argv[1]
        print(f"Hello, {name}!")
    else:
        print("Hello, World!")

if __name__ == "__main__":
    main()
"#;
        fs::write(plugin_path.join("hello.py"), script_content)?;

        // Load the new plugin
        self.load_plugin(&plugin_path)?;

        println!("Installed plugin: {}", plugin_name.green());
        Ok(())
    }

    pub fn uninstall_plugin(&mut self, name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get(name) {
            let plugin_path = &plugin.path;
            if plugin_path.exists() {
                fs::remove_dir_all(plugin_path)?;
            }
            self.plugins.remove(name);
            println!("Uninstalled plugin: {}", name.yellow());
        }

        Ok(())
    }

    pub fn enable_plugin_command(&mut self, plugin_name: &str, command_name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(plugin_name) {
            for cmd in &mut plugin.manifest.commands {
                if cmd.name == command_name {
                    cmd.enabled = true;
                    return Ok(());
                }
            }
        }

        Err(anyhow::anyhow!("Plugin command not found"))
    }

    pub fn disable_plugin_command(&mut self, plugin_name: &str, command_name: &str) -> Result<()> {
        if let Some(plugin) = self.plugins.get_mut(plugin_name) {
            for cmd in &mut plugin.manifest.commands {
                if cmd.name == command_name {
                    cmd.enabled = false;
                    return Ok(());
                }
            }
        }

        Err(anyhow::anyhow!("Plugin command not found"))
    }

    fn get_plugin_directory() -> Result<PathBuf> {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("terminal-emulator");
        path.push("plugins");
        Ok(path)
    }
}

impl Default for PluginManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            eprintln!("Failed to initialize plugin manager");
            PluginManager {
                plugins: HashMap::new(),
                plugin_dir: PathBuf::from("."),
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_manager_creation() {
        let manager = PluginManager::new();
        assert!(manager.is_ok());
    }

    #[test]
    fn test_plugin_manifest_parsing() {
        let manifest_content = r#"
name = "test"
version = "1.0.0"
description = "Test plugin"
author = "Test Author"
commands = [
    { name = "test", description = "Test command", usage = "test", script = "test.py", language = "python", enabled = true }
]
"#;

        let manifest: PluginManifest = toml::from_str(manifest_content).unwrap();
        assert_eq!(manifest.name, "test");
        assert_eq!(manifest.commands.len(), 1);
    }
} 
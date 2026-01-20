# Changelog

All notable changes to Macbrew will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.2] - 2026-01-21

### Added
- **History Export Command**:
  - New `history export <filename>` command to export command history (with timestamps and exit codes) to a file
  - Useful for audits, sharing, and backups

### Changed
- Updated version numbers to 1.2.2 in `package.json` and `Cargo.toml`

---

## [1.2.1] - 2026-01-11

### Added
- **Enhanced History Command**: 
  - History now displays timestamps for each command entry
  - Added exit code indicators (✓ for success, ✗ for failure) in history output
  - New `history search <term>` command to search through command history
  - New `history clear` command to clear command history
  - History now shows last 20 entries by default (increased from 10)
  
- **Version Command**: 
  - Added `version` command to display Macbrew version information
  - Shows version number, description, and build information

- **Improved Alias Support**:
  - Aliases are now properly resolved and executed during command processing
  - Enhanced `alias` command with better output formatting
  - Improved `unalias` command with support for removing multiple aliases at once
  - Aliases are now saved to configuration file automatically

- **Enhanced Prompt Display**:
  - Prompt now uses color scheme from configuration
  - Better visual feedback with colored prompts

- **Better Error Handling**:
  - Exit codes are now properly tracked in command history
  - Error messages use color coding from configuration
  - Improved error display with colored output

### Changed
- **History Display**: 
  - History entries now show timestamps in format `YYYY-MM-DD HH:MM:SS`
  - History entries include exit code status indicators
  - Default history display increased from 10 to 20 entries

- **Help Command**:
  - Updated help text to include new commands (`version`, `history search`, `history clear`)
  - Help output now uses color coding for better readability
  - Added documentation for alias commands

- **Version Display**:
  - Startup message now shows v1.2.1
  - Version information is consistent across all files

### Improved
- **Command Execution**:
  - Better alias resolution with recursive alias support
  - Improved command parsing and execution flow
  - Enhanced error messages with color coding

- **Configuration Management**:
  - Aliases are automatically saved when created or removed
  - Better configuration persistence

- **User Experience**:
  - More informative command outputs
  - Better visual feedback with colors
  - Enhanced history search capabilities

### Technical Details
- Updated version numbers in `package.json` and `Cargo.toml` to 1.2.1
- Added `update_last_exit_code()` method to `CommandHistory` for tracking command exit codes
- Enhanced `show_history()` method with search and clear functionality
- Improved alias handling in command execution pipeline
- Added color support to prompt display using configuration color scheme

---

## [1.0.1] - Previous Release

### Features
- Real Python execution
- Persistent command history
- Intelligent autocomplete
- Job control
- Extensible plugin system
- Rich color output
- Comprehensive file & system operations
- Homebrew simulation
- Cross-platform support


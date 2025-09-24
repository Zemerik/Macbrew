use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Read, Write};
use std::process::Command;
use std::collections::HashMap;
use anyhow::Result;
use regex::Regex;
use base64::{Engine as _, engine::general_purpose};
use md5;
use sha2::{Sha256, Sha512, Digest};
use hex;
use walkdir::WalkDir;
use tempfile::NamedTempFile;
use indicatif::{ProgressBar, ProgressStyle};

pub struct FileUtils;

impl FileUtils {
    pub fn read_file(path: &Path) -> Result<String> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    pub fn write_file(path: &Path, content: &str) -> Result<()> {
        let mut file = fs::File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn append_file(path: &Path, content: &str) -> Result<()> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    pub fn copy_file(src: &Path, dst: &Path) -> Result<u64> {
        fs::copy(src, dst).map_err(|e| anyhow::anyhow!("Failed to copy file: {}", e))
    }

    pub fn move_file(src: &Path, dst: &Path) -> Result<()> {
        fs::rename(src, dst).map_err(|e| anyhow::anyhow!("Failed to move file: {}", e))
    }

    pub fn delete_file(path: &Path) -> Result<()> {
        fs::remove_file(path).map_err(|e| anyhow::anyhow!("Failed to delete file: {}", e))
    }

    pub fn create_directory(path: &Path) -> Result<()> {
        fs::create_dir_all(path).map_err(|e| anyhow::anyhow!("Failed to create directory: {}", e))
    }

    pub fn delete_directory(path: &Path) -> Result<()> {
        fs::remove_dir_all(path).map_err(|e| anyhow::anyhow!("Failed to delete directory: {}", e))
    }

    pub fn list_directory(path: &Path) -> Result<Vec<PathBuf>> {
        let mut entries = Vec::new();
        for entry in fs::read_dir(path)? {
            entries.push(entry?.path());
        }
        Ok(entries)
    }

    pub fn find_files(pattern: &str, start_path: &Path) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        let regex = Regex::new(pattern)?;

        for entry in WalkDir::new(start_path) {
            let entry = entry?;
            if let Some(name) = entry.file_name().to_str() {
                if regex.is_match(name) {
                    files.push(entry.path().to_path_buf());
                }
            }
        }

        Ok(files)
    }

    pub fn get_file_size(path: &Path) -> Result<u64> {
        let metadata = fs::metadata(path)?;
        Ok(metadata.len())
    }

    pub fn get_file_info(path: &Path) -> Result<FileInfo> {
        let metadata = fs::metadata(path)?;
        let created = metadata.created().ok();
        let modified = metadata.modified().ok();
        let accessed = metadata.accessed().ok();

        Ok(FileInfo {
            path: path.to_path_buf(),
            size: metadata.len(),
            is_file: metadata.is_file(),
            is_dir: metadata.is_dir(),
            is_symlink: metadata.file_type().is_symlink(),
            permissions: metadata.permissions(),
            created,
            modified,
            accessed,
        })
    }

    pub fn create_temp_file() -> Result<NamedTempFile> {
        NamedTempFile::new().map_err(|e| anyhow::anyhow!("Failed to create temp file: {}", e))
    }

    pub fn create_temp_directory() -> Result<PathBuf> {
        tempfile::tempdir()
            .map(|dir| dir.path().to_path_buf())
            .map_err(|e| anyhow::anyhow!("Failed to create temp directory: {}", e))
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub path: PathBuf,
    pub size: u64,
    pub is_file: bool,
    pub is_dir: bool,
    pub is_symlink: bool,
    pub permissions: std::fs::Permissions,
    pub created: Option<std::time::SystemTime>,
    pub modified: Option<std::time::SystemTime>,
    pub accessed: Option<std::time::SystemTime>,
}

pub struct TextUtils;

impl TextUtils {
    pub fn count_lines(text: &str) -> usize {
        text.lines().count()
    }

    pub fn count_words(text: &str) -> usize {
        text.split_whitespace().count()
    }

    pub fn count_chars(text: &str) -> usize {
        text.chars().count()
    }

    pub fn count_bytes(text: &str) -> usize {
        text.len()
    }

    pub fn reverse_text(text: &str) -> String {
        text.chars().rev().collect()
    }

    pub fn to_uppercase(text: &str) -> String {
        text.to_uppercase()
    }

    pub fn to_lowercase(text: &str) -> String {
        text.to_lowercase()
    }

    pub fn capitalize(text: &str) -> String {
        let mut chars = text.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    pub fn title_case(text: &str) -> String {
        text.split_whitespace()
            .map(|word| Self::capitalize(word))
            .collect::<Vec<_>>()
            .join(" ")
    }

    pub fn remove_duplicates(text: &str) -> String {
        let mut seen = std::collections::HashSet::new();
        text.lines()
            .filter(|line| seen.insert(*line))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn sort_lines(text: &str) -> String {
        let mut lines: Vec<&str> = text.lines().collect();
        lines.sort();
        lines.join("\n")
    }

    pub fn sort_lines_reverse(text: &str) -> String {
        let mut lines: Vec<&str> = text.lines().collect();
        lines.sort_by(|a, b| b.cmp(a));
        lines.join("\n")
    }

    pub fn grep(text: &str, pattern: &str) -> Result<Vec<String>> {
        let regex = Regex::new(pattern)?;
        let matches: Vec<String> = text
            .lines()
            .filter(|line| regex.is_match(line))
            .map(|line| line.to_string())
            .collect();
        Ok(matches)
    }

    pub fn replace_all(text: &str, pattern: &str, replacement: &str) -> Result<String> {
        let regex = Regex::new(pattern)?;
        Ok(regex.replace_all(text, replacement).to_string())
    }

    pub fn extract_matches(text: &str, pattern: &str) -> Result<Vec<String>> {
        let regex = Regex::new(pattern)?;
        let matches: Vec<String> = regex
            .find_iter(text)
            .map(|m| m.as_str().to_string())
            .collect();
        Ok(matches)
    }
}

pub struct CryptoUtils;

impl CryptoUtils {
    pub fn md5_hash(data: &str) -> String {
        let digest = md5::compute(data.as_bytes());
        format!("{:x}", digest)
    }

    pub fn sha256_hash(data: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn sha512_hash(data: &str) -> String {
        let mut hasher = Sha512::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn base64_encode(data: &str) -> String {
        general_purpose::STANDARD.encode(data.as_bytes())
    }

    pub fn base64_decode(data: &str) -> Result<String> {
        let decoded = general_purpose::STANDARD.decode(data)?;
        String::from_utf8(decoded).map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
    }

    pub fn hex_encode(data: &str) -> String {
        hex::encode(data.as_bytes())
    }

    pub fn hex_decode(data: &str) -> Result<String> {
        let decoded = hex::decode(data)?;
        String::from_utf8(decoded).map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
    }

    pub fn generate_random_string(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(*&^%$#@!~";
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }

    pub fn generate_random_password(length: usize) -> String {
        use rand::Rng;
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789!@#$%^&*()_+-=[]{}|;:,.<>?";
        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect()
    }
}

pub struct SystemUtils;

impl SystemUtils {
    pub fn get_system_info() -> SystemInfo {
        let os_info = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let hostname = std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string());
        let username = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
        let home_dir = std::env::var("HOME").unwrap_or_else(|_| "unknown".to_string());
        let current_dir = std::env::current_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        SystemInfo {
            os: os_info.to_string(),
            architecture: arch.to_string(),
            hostname,
            username,
            home_directory: home_dir,
            current_directory: current_dir,
        }
    }

    pub fn get_environment_variables() -> HashMap<String, String> {
        std::env::vars().collect()
    }

    pub fn set_environment_variable(key: &str, value: &str) -> Result<()> {
        std::env::set_var(key, value);
        Ok(())
    }

    pub fn unset_environment_variable(key: &str) -> Result<()> {
        std::env::remove_var(key);
        Ok(())
    }

    pub fn get_process_id() -> u32 {
        std::process::id()
    }

    pub fn get_parent_process_id() -> Option<u32> {
        // This is a simplified version - on Unix systems you'd read /proc/self/stat
        None
    }

    pub fn sleep(seconds: u64) {
        std::thread::sleep(std::time::Duration::from_secs(seconds));
    }

    pub fn sleep_millis(millis: u64) {
        std::thread::sleep(std::time::Duration::from_millis(millis));
    }

    pub fn get_current_time() -> std::time::SystemTime {
        std::time::SystemTime::now()
    }

    pub fn format_duration(duration: std::time::Duration) -> String {
        let total_secs = duration.as_secs();
        let hours = total_secs / 3600;
        let minutes = (total_secs % 3600) / 60;
        let seconds = total_secs % 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }
}

#[derive(Debug)]
pub struct SystemInfo {
    pub os: String,
    pub architecture: String,
    pub hostname: String,
    pub username: String,
    pub home_directory: String,
    pub current_directory: String,
}

pub struct ProgressUtils;

impl ProgressUtils {
    pub fn create_progress_bar(len: u64) -> ProgressBar {
        let pb = ProgressBar::new(len);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb
    }

    pub fn create_spinner() -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {wide_msg}")
                .unwrap()
                .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
        );
        pb
    }

    pub fn create_indeterminate_progress() -> ProgressBar {
        let pb = ProgressBar::new(0);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb
    }
}

pub struct NetworkUtils;

impl NetworkUtils {
    pub fn ping(host: &str, count: u32) -> Result<PingResult> {
        let output = Command::new("ping")
            .arg("-c")
            .arg(count.to_string())
            .arg(host)
            .output()?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        
        // Parse ping output (simplified)
        let lines: Vec<&str> = output_str.lines().collect();
        let mut result = PingResult {
            host: host.to_string(),
            transmitted: 0,
            received: 0,
            loss_percentage: 0.0,
            min_time: 0.0,
            avg_time: 0.0,
            max_time: 0.0,
            mdev_time: 0.0,
        };

        // This is a simplified parser - real implementation would be more robust
        for line in lines {
            if line.contains("packets transmitted") {
                // Parse packet statistics
                if let Some(transmitted) = line.split_whitespace().next() {
                    result.transmitted = transmitted.parse().unwrap_or(0);
                }
            }
        }

        Ok(result)
    }

    pub fn curl(url: &str) -> Result<String> {
        let output = Command::new("curl")
            .arg("-s")
            .arg(url)
            .output()?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub fn wget(url: &str, output_file: Option<&str>) -> Result<()> {
        let mut cmd = Command::new("wget");
        cmd.arg(url);
        
        if let Some(file) = output_file {
            cmd.arg("-O").arg(file);
        }

        cmd.output()?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct PingResult {
    pub host: String,
    pub transmitted: u32,
    pub received: u32,
    pub loss_percentage: f64,
    pub min_time: f64,
    pub avg_time: f64,
    pub max_time: f64,
    pub mdev_time: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_utils() {
        assert_eq!(TextUtils::count_lines("line1\nline2\nline3"), 3);
        assert_eq!(TextUtils::count_words("hello world test"), 3);
        assert_eq!(TextUtils::reverse_text("hello"), "olleh");
    }

    #[test]
    fn test_crypto_utils() {
        let hash = CryptoUtils::md5_hash("hello");
        assert_eq!(hash.len(), 32);
        
        let encoded = CryptoUtils::base64_encode("hello");
        let decoded = CryptoUtils::base64_decode(&encoded).unwrap();
        assert_eq!(decoded, "hello");
    }

    #[test]
    fn test_system_utils() {
        let info = SystemUtils::get_system_info();
        assert!(!info.os.is_empty());
        assert!(!info.architecture.is_empty());
    }
} 
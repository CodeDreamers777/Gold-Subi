// src/config/settings.rs
use std::{error::Error, fs, path::PathBuf};

pub fn get_journal_dir() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("Documents");
    dir.push("notes");
    dir
}

pub fn get_trades_dir() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push("Documents");
    dir.push("trades");
    fs::create_dir_all(&dir).expect("Failed to create trades directory");
    dir
}

pub fn get_config_path() -> PathBuf {
    let mut dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    dir.push(".termjournal");
    fs::create_dir_all(&dir).expect("Failed to create config directory");
    dir.push("config.txt");
    dir
}

pub fn get_editor() -> Result<String, Box<dyn Error>> {
    let config_path = get_config_path();
    match fs::read_to_string(config_path) {
        Ok(editor) if !editor.trim().is_empty() => Ok(editor),
        _ => Ok("nvim".to_string()), // Default to nvim
    }
}

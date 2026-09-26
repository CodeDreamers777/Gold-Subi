// src/journal/storage.rs
use std::{error::Error, fs::{self, File}, io::Read};

use crate::{
    app::App,
    config::settings::{get_journal_dir, get_trades_dir},
    journal::entry::{JournalEntry, StatefulList, TradeEntry},
};

pub fn load_trades() -> Result<Vec<TradeEntry>, Box<dyn Error>> {
    let trades_dir = get_trades_dir();
    let mut trades = Vec::new();

    if let Ok(entries_iter) = fs::read_dir(&trades_dir) {
        for entry in entries_iter {
            if let Ok(entry) = entry {
                let path = entry.path();
                
                if path.extension().unwrap_or_default() == "yaml" {
                    if let Ok(mut file) = File::open(&path) {
                        let mut content = String::new();
                        let _ = file.read_to_string(&mut content);
                        if let Ok(trade) = serde_yaml::from_str::<TradeEntry>(&content) {
                            trades.push(trade);
                        }
                    }
                }
            }
        }
    }
    
    // Sort trades by date (newest first)
    trades.sort_by(|a, b| b.date.cmp(&a.date));
    
    Ok(trades)
}

pub fn refresh_trades(app: &mut App) -> Result<(), Box<dyn Error>> {
    let trades = load_trades()?;
    app.trades_list = crate::journal::entry::StatefulTable::with_items(trades);
    Ok(())
}

pub fn load_entries() -> Result<Vec<JournalEntry>, Box<dyn Error>> {
    let journal_dir = get_journal_dir();
    let mut entries = Vec::new();

    if let Ok(entries_iter) = fs::read_dir(&journal_dir) {
        for entry in entries_iter {
            if let Ok(entry) = entry {
                let path = entry.path();
                
                // Only process .md files
                if path.extension().unwrap_or_default() == "md" {
                    if let Some(name) = path.file_name() {
                        if let Some(name_str) = name.to_str() {
                            let date = name_str.replace(".md", "");
                            
                            // Get file metadata
                            if let Ok(metadata) = fs::metadata(&path) {
                                let size = metadata.len();
                                
                                // Read content
                                let mut content = String::new();
                                if let Ok(mut file) = File::open(&path) {
                                    let _ = file.read_to_string(&mut content);
                                }
                                
                                entries.push(JournalEntry {
                                    date,
                                    size,
                                    content,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Sort entries by date (newest first)
    entries.sort_by(|a, b| b.date.cmp(&a.date));
    
    Ok(entries)
}

pub fn refresh_entries(app: &mut App) -> Result<(), Box<dyn Error>> {
    let entries = load_entries()?;
    app.entries_list = StatefulList::with_items(entries);
    Ok(())
}

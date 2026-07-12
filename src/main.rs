// src/main.rs
use clap::{Parser, Subcommand};
use colored::*;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{error::Error, fs, io};

mod app;
mod config;
mod journal;
mod ui;

use app::App;
use chrono::Local;
use config::settings::{get_config_path, get_journal_dir};
use journal::commands::{delete_entry_cli, display_entries_list, open_editor, view_entry_cli};

#[derive(Parser)]
#[command(name = "termjournal")]
#[command(about = "A beautiful CLI journal app 📝", long_about = None)]
#[command(after_help = "Run without arguments to use the interactive TUI")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new journal entry
    New,

    /// View a journal entry
    View {
        /// Date in YYYY-MM-DD format (defaults to today)
        date: Option<String>,
    },

    /// Delete a journal entry
    Delete {
        /// Date in YYYY-MM-DD format (defaults to today)
        date: Option<String>,
    },

    /// List all journal entries
    List,

    /// Configure your journal settings
    Config {
        /// Set editor, e.g., nvim, vim, nano, code
        #[arg(help = "Set editor, e.g., nvim, vim, nano, code")]
        editor: String,
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let journal_dir = get_journal_dir();
    fs::create_dir_all(&journal_dir)?;

    match cli.command {
        Some(Commands::New) => {
            let date = Local::now().format("%Y-%m-%d").to_string();
            let path = journal_dir.join(format!("{date}.md"));
            open_editor(&path)?;
            Ok(())
        }
        Some(Commands::View { date }) => {
            let date = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
            view_entry_cli(&date)?;
            Ok(())
        }
        Some(Commands::Delete { date }) => {
            let date = date.unwrap_or_else(|| Local::now().format("%Y-%m-%d").to_string());
            delete_entry_cli(&date)?;
            Ok(())
        }
        Some(Commands::List) => {
            display_entries_list()?;
            Ok(())
        }
        Some(Commands::Config { editor }) => {
            let config_path = get_config_path();
            fs::write(config_path, editor)?;
            println!("{}", "✓ Editor updated!".green());
            Ok(())
        }
        None => {
            // Initialize terminal
            enable_raw_mode()?;
            let mut stdout = io::stdout();
            execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
            let backend = CrosstermBackend::new(stdout);
            let mut terminal = Terminal::new(backend)?;

            // Create app state
            let mut app = App::new()?;

            // Main loop
            let res = app::run_app(&mut terminal, &mut app);

            // Restore terminal
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;

            if let Err(err) = res {
                println!("{}", format!("Error: {}", err).red());
            }

            Ok(())
        }
    }
}

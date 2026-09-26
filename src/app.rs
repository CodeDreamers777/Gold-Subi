// src/app.rs
use std::{error::Error, time::{Duration, Instant}};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

use crate::{
    config::settings::get_editor,
    journal::{
        commands::{create_new_entry, delete_entry, open_entry},
        storage::{load_entries, refresh_entries, load_trades},
    },
    ui::ui,
};

pub enum InputMode {
    Normal,
}

use std::sync::{Arc, Mutex};
use std::thread;

pub struct App {
    pub tab_index: usize,
    pub entries_list: crate::journal::entry::StatefulList<crate::journal::entry::JournalEntry>,
    pub trades_list: crate::journal::entry::StatefulTable<crate::journal::entry::TradeEntry>,
    pub input_mode: InputMode,
    pub show_help: bool,
    pub editor: String,
    pub status_message: String,
    pub status_time: Option<Instant>,
    pub ticker_data: Arc<Mutex<String>>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let entries = load_entries()?;
        let trades = load_trades()?;
        let editor = get_editor()?;

        let ticker_data = Arc::new(Mutex::new(String::from("Loading Live Ticker...")));
        let ticker_clone = Arc::clone(&ticker_data);

        // Background thread for live ticker
        thread::spawn(move || {
            loop {
                // Fetch prices (Binance API for Forex/Crypto proxies)
                let pairs = ["EURUSDT", "GBPUSDT", "XAUUSDT", "BTCUSDT"];
                let mut prices = String::new();
                for pair in &pairs {
                    if let Ok(resp) = ureq::get(&format!("https://api.binance.com/api/v3/ticker/price?symbol={}", pair)).call() {
                        if let Ok(json) = resp.into_string() {
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json) {
                                if let Some(price) = parsed["price"].as_str() {
                                    if let Ok(p) = price.parse::<f64>() {
                                        prices.push_str(&format!("{} {:.4}   ", pair.replace("USDT", "/USD"), p));
                                    }
                                }
                            }
                        }
                    }
                }
                if !prices.is_empty() {
                    if let Ok(mut lock) = ticker_clone.lock() {
                        *lock = prices;
                    }
                }
                thread::sleep(Duration::from_secs(10));
            }
        });

        Ok(App {
            tab_index: 0,
            entries_list: crate::journal::entry::StatefulList::with_items(entries),
            trades_list: crate::journal::entry::StatefulTable::with_items(trades),
            input_mode: InputMode::Normal,
            show_help: false,
            editor,
            status_message: String::new(),
            status_time: None,
            ticker_data,
        })
    }

    pub fn set_status(&mut self, message: &str) {
        self.status_message = message.to_string();
        self.status_time = Some(Instant::now());
    }
}

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui::ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('h') => app.show_help = !app.show_help,
                            KeyCode::Char('j') | KeyCode::Down => {
                                if app.tab_index == 0 { app.entries_list.next(); }
                                else if app.tab_index == 1 { app.trades_list.next(); }
                            },
                            KeyCode::Char('k') | KeyCode::Up => {
                                if app.tab_index == 0 { app.entries_list.previous(); }
                                else if app.tab_index == 1 { app.trades_list.previous(); }
                            },
                            KeyCode::Char('n') => {
                                crossterm::terminal::disable_raw_mode()?;
                                crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
                                terminal.show_cursor()?;

                                if app.tab_index == 0 {
                                    crate::journal::commands::create_new_entry(app)?;
                                    crossterm::terminal::enable_raw_mode()?;
                                    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
                                    terminal.clear()?;
                                    crate::journal::storage::refresh_entries(app)?;
                                } else if app.tab_index == 1 {
                                    crate::journal::commands::create_new_trade(app)?;
                                    crossterm::terminal::enable_raw_mode()?;
                                    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
                                    terminal.clear()?;
                                    crate::journal::storage::refresh_trades(app)?;
                                } else {
                                    crossterm::terminal::enable_raw_mode()?;
                                    crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
                                }
                            }
                            KeyCode::Enter => {
                                if app.tab_index == 0 {
                                    if let Some(selected) = app.entries_list.state.selected() {
                                        if !app.entries_list.items.is_empty() {
                                            let date = app.entries_list.items[selected].date.clone();
                                            crossterm::terminal::disable_raw_mode()?;
                                            crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
                                            terminal.show_cursor()?;
                                            crate::journal::commands::open_entry(&date)?;
                                            crossterm::terminal::enable_raw_mode()?;
                                            crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
                                            terminal.clear()?;
                                            crate::journal::storage::refresh_entries(app)?;
                                        }
                                    }
                                } else if app.tab_index == 1 {
                                    if let Some(selected) = app.trades_list.state.selected() {
                                        if !app.trades_list.items.is_empty() {
                                            let id = app.trades_list.items[selected].id.clone();
                                            crossterm::terminal::disable_raw_mode()?;
                                            crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
                                            terminal.show_cursor()?;
                                            crate::journal::commands::open_trade(&id)?;
                                            crossterm::terminal::enable_raw_mode()?;
                                            crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
                                            terminal.clear()?;
                                            crate::journal::storage::refresh_trades(app)?;
                                        }
                                    }
                                }
                            }
                            KeyCode::Char('d') => {
                                if app.tab_index == 0 {
                                    if let Some(selected) = app.entries_list.state.selected() {
                                        if !app.entries_list.items.is_empty() {
                                            let date = app.entries_list.items[selected].date.clone();
                                            crate::journal::commands::delete_entry(date, app)?;
                                            crate::journal::storage::refresh_entries(app)?;
                                        }
                                    }
                                } else if app.tab_index == 1 {
                                    if let Some(selected) = app.trades_list.state.selected() {
                                        if !app.trades_list.items.is_empty() {
                                            let id = app.trades_list.items[selected].id.clone();
                                            crate::journal::commands::delete_trade(id, app)?;
                                            crate::journal::storage::refresh_trades(app)?;
                                        }
                                    }
                                }
                            }
                            KeyCode::Tab => {
                                app.tab_index = (app.tab_index + 1) % 5;
                            }
                            KeyCode::BackTab => {
                                app.tab_index = if app.tab_index > 0 {
                                    app.tab_index - 1
                                } else {
                                    4
                                };
                            }
                            _ => {}
                        },
                    }
                }
            }
        }

        // Clear status message after timeout
        if let Some(status_time) = app.status_time {
            if status_time.elapsed() > Duration::from_secs(3) {
                app.status_message = String::new();
                app.status_time = None;
            }
        }
    }
}

// src/app.rs
use std::{error::Error, time::{Duration, Instant}};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::Backend, Terminal};

use crate::{
    config::settings::get_editor,
    journal::{
        commands::{create_new_entry, delete_entry, open_entry},
        storage::{load_entries, refresh_entries},
    },
    ui::ui,
};

pub enum InputMode {
    Normal,
}

pub struct App {
    pub tab_index: usize,
    pub entries_list: crate::journal::entry::StatefulList<crate::journal::entry::JournalEntry>,
    pub input_mode: InputMode,
    pub show_help: bool,
    pub editor: String,
    pub status_message: String,
    pub status_time: Option<Instant>,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let entries = load_entries()?;
        let editor = get_editor()?;

        Ok(App {
            tab_index: 0,
            entries_list: crate::journal::entry::StatefulList::with_items(entries),
            input_mode: InputMode::Normal,
            show_help: false,
            editor,
            status_message: String::new(),
            status_time: None,
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
                            KeyCode::Char('j') | KeyCode::Down => app.entries_list.next(),
                            KeyCode::Char('k') | KeyCode::Up => app.entries_list.previous(),
                            KeyCode::Char('n') => {
                                crossterm::terminal::disable_raw_mode()?;
                                crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
                                terminal.show_cursor()?;

                                create_new_entry(app)?;

                                crossterm::terminal::enable_raw_mode()?;
                                crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
                                terminal.clear()?;
                                refresh_entries(app)?;
                            }
                            KeyCode::Enter => {
                                if let Some(selected) = app.entries_list.state.selected() {
                                    if !app.entries_list.items.is_empty() {
                                        let date = app.entries_list.items[selected].date.clone();
                                        
                                        crossterm::terminal::disable_raw_mode()?;
                                        crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen, crossterm::event::DisableMouseCapture)?;
                                        terminal.show_cursor()?;

                                        open_entry(&date)?;

                                        crossterm::terminal::enable_raw_mode()?;
                                        crossterm::execute!(std::io::stdout(), crossterm::terminal::EnterAlternateScreen, crossterm::event::EnableMouseCapture)?;
                                        terminal.clear()?;
                                        refresh_entries(app)?;
                                    }
                                }
                            }
                            KeyCode::Char('d') => {
                                if let Some(selected) = app.entries_list.state.selected() {
                                    if !app.entries_list.items.is_empty() {
                                        let date = app.entries_list.items[selected].date.clone();
                                        delete_entry(date, app)?;
                                        refresh_entries(app)?;
                                    }
                                }
                            }
                            KeyCode::Tab => {
                                app.tab_index = (app.tab_index + 1) % 3;
                            }
                            KeyCode::BackTab => {
                                app.tab_index = if app.tab_index > 0 {
                                    app.tab_index - 1
                                } else {
                                    2
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

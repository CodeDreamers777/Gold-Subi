// src/journal/entry.rs
use ratatui::widgets::{ListState, TableState};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct TradeEntry {
    pub id: String,
    pub date: String,
    pub pair: String,
    pub direction: String,
    pub entry_price: f64,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub exit_price: Option<f64>,
    pub lot_size: f64,
    pub pnl: Option<f64>,
    pub setup: String,
    pub session: String,
    pub notes: String,
}

pub struct JournalEntry {
    pub date: String,
    pub size: u64,
    pub content: String,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        let mut state = ListState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        StatefulList { state, items }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

pub struct StatefulTable<T> {
    pub state: TableState,
    pub items: Vec<T>,
}

impl<T> StatefulTable<T> {
    pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
        let mut state = TableState::default();
        if !items.is_empty() {
            state.select(Some(0));
        }
        StatefulTable { state, items }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

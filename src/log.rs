use std::net::SocketAddr;

use chrono::{DateTime, Local};
use ratatui::widgets::{ListItem, ListState};
use rosc::OscPacket;

pub struct LogEntry(pub DateTime<Local>, pub OscPacket, pub SocketAddr);

impl LogEntry {
    pub fn to_list_item(&self, i: usize) -> ListItem {
        let line = format!("{:?}", self.1);

        ListItem::new(line)
    }
}

pub struct StatefulLog {
    pub state: ListState,
    pub items: Vec<LogEntry>,
    pub last_selected: Option<usize>,
}

impl Default for StatefulLog {
    fn default() -> Self {
        Self {
            state: ListState::default(),
            items: vec![],
            last_selected: None,
        }
    }
}

impl StatefulLog {
    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.last_selected.unwrap_or(0),
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
            None => self.last_selected.unwrap_or(0),
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        let offset = self.state.offset();
        self.last_selected = self.state.selected();
        self.state.select(None);
        *self.state.offset_mut() = offset;
    }
}

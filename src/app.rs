use ratatui::widgets::TableState;

use crate::config::Entry;
use crate::theme::Theme;

pub struct App {
    pub entries: Vec<Entry>,
    pub filter: String,
    pub filtered: Vec<usize>,
    pub table_state: TableState,
    pub theme: Theme,
    pub show_notes: bool,
}

impl App {
    pub fn new(entries: Vec<Entry>, theme: Theme) -> Self {
        let filtered = (0..entries.len()).collect();
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        Self {
            entries,
            filter: String::new(),
            filtered,
            table_state,
            theme,
            show_notes: false,
        }
    }

    pub fn apply_filter(&mut self) {
        let needle = self.filter.to_lowercase();
        self.filtered = self
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                needle.is_empty()
                    || e.name.to_lowercase().contains(&needle)
                    || e.command.to_lowercase().contains(&needle)
                    || e.notes.to_lowercase().contains(&needle)
            })
            .map(|(i, _)| i)
            .collect();
        let max = self.filtered.len().saturating_sub(1);
        let sel = self.table_state.selected().unwrap_or(0).min(max);
        self.table_state.select(Some(sel));
    }

    pub fn move_selection(&mut self, delta: i32) {
        if self.filtered.is_empty() {
            return;
        }
        let len = self.filtered.len() as i32;
        let cur = self.table_state.selected().unwrap_or(0) as i32;
        let next = ((cur + delta).rem_euclid(len)) as usize;
        self.table_state.select(Some(next));
    }

    pub fn selected_entry(&self) -> Option<&Entry> {
        self.table_state
            .selected()
            .and_then(|i| self.filtered.get(i))
            .and_then(|&idx| self.entries.get(idx))
    }
}

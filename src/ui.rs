use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Cell, Clear, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::App;

pub fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(3),
        ])
        .split(f.size());

    let theme = &app.theme;

    let input = Paragraph::new(app.filter.as_str()).block(theme.block());
    f.render_widget(input, chunks[0]);

    let header_style = Style::default()
        .fg(theme.header)
        .add_modifier(Modifier::BOLD);
    let header = Row::new(vec![Cell::from("Name"), Cell::from("Command")]).style(header_style);

    let rows: Vec<Row> = app
        .filtered
        .iter()
        .map(|&idx| {
            let e = &app.entries[idx];
            Row::new(vec![
                Cell::from(e.name.replace('\n', "; ")),
                Cell::from(e.command.replace('\n', "; ")),
            ])
        })
        .collect();

    let table = Table::new(rows, [Constraint::Percentage(30), Constraint::Percentage(70)])
        .header(header)
        .block(theme.block())
        .highlight_style(Style::default().bg(theme.highlight_bg).fg(theme.highlight_fg))
        .highlight_symbol("");

    f.render_stateful_widget(table, chunks[1], &mut app.table_state);

    let footer = Paragraph::new(Line::from(vec![Span::raw(
        "(Esc) quit | (\u{2191}) up | (\u{2193}) down | (enter) run | (tab) notes",
    )]))
    .alignment(Alignment::Center)
    .block(theme.block());
    f.render_widget(footer, chunks[2]);

    if app.show_notes {
        if let Some(e) = app.selected_entry() {
            let area = centered_rect(60, 40, f.size());
            let notes = if e.notes.is_empty() {
                "(no notes)".to_string()
            } else {
                e.notes.clone()
            };
            let popup = Paragraph::new(format!("{notes}\n\n(Esc or tab to close)"))
                .wrap(Wrap { trim: false })
                .block(theme.block().title(format!(" {} ", e.name)));
            f.render_widget(Clear, area);
            f.render_widget(popup, area);
        }
    }
}

/// A Rect centered within `area`, `percent_x`/`percent_y` of its size.
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

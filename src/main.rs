use std::{
    env, fs,
    io::{self, Stdout, Write},
    process::Command,
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Cell, Padding, Paragraph, Row, Table, TableState},
    Frame, Terminal,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
struct Entry {
    name: String,
    command: String,
    #[serde(default)]
    notes: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    entries: Vec<Entry>,
    #[serde(default)]
    theme: ThemeConfig,
}

/// The `[theme]` table in config.toml. Colors are "#rrggbb" hex strings.
#[derive(Debug, Deserialize)]
#[serde(default)]
struct ThemeConfig {
    border: String,
    header: String,
    highlight_bg: String,
    highlight_fg: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            border: "#b48cff".into(),
            header: "#b48cff".into(),
            highlight_bg: "#463764".into(),
            highlight_fg: "#ffffff".into(),
        }
    }
}

fn parse_hex_color(s: &str) -> Color {
    let s = s.trim().trim_start_matches('#');
    // Length + is_ascii checks up front mean every subsequent byte slice
    // is guaranteed in-bounds and on a char boundary — no panics on "",
    // "#ab", or multi-byte input like "#aé".
    let parsed = (s.len() == 6 && s.is_ascii())
        .then(|| {
            let r = u8::from_str_radix(&s[0..2], 16);
            let g = u8::from_str_radix(&s[2..4], 16);
            let b = u8::from_str_radix(&s[4..6], 16);
            match (r, g, b) {
                (Ok(r), Ok(g), Ok(b)) => Some(Color::Rgb(r, g, b)),
                _ => None,
            }
        })
        .flatten();

    parsed.unwrap_or_else(|| {
        eprintln!("sfav: couldn't parse color \"{s}\", falling back to white");
        Color::White
    })
}

/// Runtime colors, resolved once from ThemeConfig at startup.
struct Theme {
    border: Color,
    header: Color,
    highlight_bg: Color,
    highlight_fg: Color,
}

impl Theme {
    fn from_config(cfg: &ThemeConfig) -> Self {
        Self {
            border: parse_hex_color(&cfg.border),
            header: parse_hex_color(&cfg.header),
            highlight_bg: parse_hex_color(&cfg.highlight_bg),
            highlight_fg: parse_hex_color(&cfg.highlight_fg),
        }
    }

    /// A bordered, rounded-corner block using the theme's border color,
    /// with a little breathing room so text doesn't sit flush against
    /// the border (matches sshs's spacing).
    fn block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.border))
            .padding(Padding::horizontal(1))
    }
}

fn print_help() {
    println!(
        "sfav — a minimal TUI launcher for shell commands\n\n\
         Usage: sfav [CONFIG_PATH]\n\n\
         CONFIG_PATH defaults to $XDG_CONFIG_HOME/sfav/config.toml, or\n\
         ~/.config/sfav/config.toml if that's unset."
    );
}

fn config_path() -> String {
    if let Some(arg) = env::args().nth(1) {
        if arg == "-h" || arg == "--help" {
            print_help();
            std::process::exit(0);
        }
        return arg;
    }
    if let Ok(xdg) = env::var("XDG_CONFIG_HOME") {
        if !xdg.is_empty() {
            return format!("{xdg}/sfav/config.toml");
        }
    }
    let home = env::var("HOME").unwrap_or_else(|_| ".".into());
    format!("{home}/.config/sfav/config.toml")
}

fn load_config(path: &str) -> Config {
    let text = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("couldn't read config at {path}: {e}");
        std::process::exit(1);
    });
    toml::from_str(&text).unwrap_or_else(|e| {
        eprintln!("bad config: {e}");
        std::process::exit(1);
    })
}

struct App {
    entries: Vec<Entry>,
    filter: String,
    filtered: Vec<usize>,
    table_state: TableState,
    theme: Theme,
}

impl App {
    fn new(entries: Vec<Entry>, theme: Theme) -> Self {
        let filtered = (0..entries.len()).collect();
        let mut table_state = TableState::default();
        table_state.select(Some(0));
        Self {
            entries,
            filter: String::new(),
            filtered,
            table_state,
            theme,
        }
    }

    fn apply_filter(&mut self) {
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

    fn move_selection(&mut self, delta: i32) {
        if self.filtered.is_empty() {
            return;
        }
        let len = self.filtered.len() as i32;
        let cur = self.table_state.selected().unwrap_or(0) as i32;
        let next = ((cur + delta).rem_euclid(len)) as usize;
        self.table_state.select(Some(next));
    }

    fn selected_entry(&self) -> Option<&Entry> {
        self.table_state
            .selected()
            .and_then(|i| self.filtered.get(i))
            .and_then(|&idx| self.entries.get(idx))
    }
}

fn ui(f: &mut Frame, app: &mut App) {
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
    let header = Row::new(vec![
        Cell::from("Name"),
        Cell::from("Command"),
        Cell::from("Notes"),
    ])
    .style(header_style);

    let rows: Vec<Row> = app
        .filtered
        .iter()
        .map(|&idx| {
            let e = &app.entries[idx];
            Row::new(vec![
                Cell::from(e.name.replace('\n', "; ")),
                Cell::from(e.command.replace('\n', "; ")),
                Cell::from(e.notes.replace('\n', "; ")),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(25),
            Constraint::Percentage(40),
            Constraint::Percentage(35),
        ],
    )
    .header(header)
    .block(theme.block())
    .highlight_style(Style::default().bg(theme.highlight_bg).fg(theme.highlight_fg))
    .highlight_symbol("");

    f.render_stateful_widget(table, chunks[1], &mut app.table_state);

    let footer = Paragraph::new(Line::from(vec![Span::raw(
        "(Esc) quit | (\u{2191}) up | (\u{2193}) down | (enter) run",
    )]))
    .alignment(Alignment::Center)
    .block(theme.block());
    f.render_widget(footer, chunks[2]);
}

enum Action {
    Quit,
    Run(String),
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<Action> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Release {
                    continue;
                }
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)
                {
                    return Ok(Action::Quit);
                }
                match key.code {
                    KeyCode::Esc => return Ok(Action::Quit),
                    KeyCode::Enter => {
                        if let Some(e) = app.selected_entry() {
                            return Ok(Action::Run(e.command.clone()));
                        }
                    }
                    KeyCode::Up => app.move_selection(-1),
                    KeyCode::Down => app.move_selection(1),
                    KeyCode::Backspace => {
                        app.filter.pop();
                        app.apply_filter();
                    }
                    KeyCode::Char(c) => {
                        app.filter.push(c);
                        app.apply_filter();
                    }
                    _ => {}
                }
            }
        }
    }
}

/// Drop out of the TUI, run `command` with real stdio so its output is
/// visible, wait for a keypress, then hand the terminal back to the caller.
fn run_command_and_wait(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    command: &str,
) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    println!("$ {command}");
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".into());
    let status = Command::new(&shell).arg("-c").arg(command).status();

    // Only make the user press enter if something went wrong — on a clean
    // exit, drop straight back into the picker.
    let needs_pause = match &status {
        Ok(s) => !s.success(),
        Err(_) => true,
    };
    match &status {
        Ok(s) if needs_pause => println!("\n[sfav] exited: {s} \u{2014} press enter to return"),
        Err(e) => println!("\n[sfav] failed to run: {e} \u{2014} press enter to return"),
        _ => {}
    }
    if needs_pause {
        io::stdout().flush()?;
        let mut discard = String::new();
        io::stdin().read_line(&mut discard)?;
    }

    enable_raw_mode()?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;
    terminal.clear()?;
    Ok(())
}

fn main() -> io::Result<()> {
    let cfg = load_config(&config_path());
    let theme = Theme::from_config(&cfg.theme);
    let mut app = App::new(cfg.entries, theme);

    // If anything panics while the TUI is up, restore the terminal first —
    // otherwise a panic leaves the terminal stuck in raw/alternate-screen
    // mode until the user blindly types `reset`.
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);
        default_hook(info);
    }));

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let outcome = loop {
        match run_app(&mut terminal, &mut app) {
            Ok(Action::Quit) => break Ok(()),
            Ok(Action::Run(command)) => {
                if let Err(e) = run_command_and_wait(&mut terminal, &command) {
                    break Err(e);
                }
            }
            Err(e) => break Err(e),
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    outcome
}

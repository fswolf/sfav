mod app;
mod cli;
mod config;
mod event;
mod theme;
mod ui;

use std::io;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::App;
use cli::config_path;
use config::load_config;
use event::{run_app, run_command_and_wait, Action};
use theme::Theme;

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
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
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
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    outcome
}

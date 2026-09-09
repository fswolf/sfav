use std::{
    env,
    io::{self, Stdout, Write},
    process::Command,
    time::Duration,
};

use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind,
        KeyModifiers, MouseEventKind,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::app::App;
use crate::ui::ui;

pub enum Action {
    Quit,
    Run(String),
}

pub fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> io::Result<Action> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(200))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Release {
                        continue;
                    }
                    if key.code == KeyCode::Char('c')
                        && key.modifiers.contains(KeyModifiers::CONTROL)
                    {
                        return Ok(Action::Quit);
                    }
                    if app.show_notes {
                        match key.code {
                            KeyCode::Esc | KeyCode::Tab => app.show_notes = false,
                            _ => {}
                        }
                        continue;
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
                        KeyCode::Tab => app.show_notes = true,
                        KeyCode::Char(c) => {
                            app.filter.push(c);
                            app.apply_filter();
                        }
                        _ => {}
                    }
                }
                // One row per notch, both directions. Every other mouse
                // event (clicks, drags, moves) is left alone — the
                // terminal's synthetic click reporting isn't something we
                // want to act on, and text selection still works via
                // Shift+drag, which every terminal honors regardless of
                // whether the app has mouse reporting on.
                Event::Mouse(m) => match m.kind {
                    MouseEventKind::ScrollDown => app.move_selection(1),
                    MouseEventKind::ScrollUp => app.move_selection(-1),
                    _ => {}
                },
                _ => {}
            }
        }
    }
}

/// Drop out of the TUI, run `command` with real stdio so its output is
/// visible, wait for a keypress, then hand the terminal back to the caller.
pub fn run_command_and_wait(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    command: &str,
) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
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
    execute!(
        terminal.backend_mut(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    terminal.clear()?;
    Ok(())
}

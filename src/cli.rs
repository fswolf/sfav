use std::env;

fn print_help() {
    println!(
        "sfav — a minimal TUI launcher for shell commands\n\n\
         Usage: sfav [CONFIG_PATH]\n\
                sfav --version | -v\n\
                sfav --help | -h\n\n\
         CONFIG_PATH defaults to $XDG_CONFIG_HOME/sfav/config.toml, or\n\
         ~/.config/sfav/config.toml if that's unset."
    );
}

/// Resolves the config path from argv, or the default location. Exits the
/// process directly for -h/--help and -v/--version since neither needs a
/// config loaded at all.
pub fn config_path() -> String {
    if let Some(arg) = env::args().nth(1) {
        if arg == "-h" || arg == "--help" {
            print_help();
            std::process::exit(0);
        }
        if arg == "-v" || arg == "--version" {
            println!("sfav {}", env!("CARGO_PKG_VERSION"));
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

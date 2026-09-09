use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Entry {
    pub name: String,
    pub command: String,
    #[serde(default)]
    pub notes: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub entries: Vec<Entry>,
    #[serde(default)]
    pub theme: ThemeConfig,
}

/// The `[theme]` table in config.toml. Colors are "#rrggbb" hex strings.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct ThemeConfig {
    pub border: String,
    pub header: String,
    pub highlight_bg: String,
    pub highlight_fg: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            border: "#60a5fa".into(),
            header: "#06b6d4".into(),
            highlight_bg: "#d4d4d8".into(),
            highlight_fg: "#18181b".into(),
        }
    }
}

/// The example config, embedded at compile time so the binary can always
/// bootstrap a working config on first run, no matter how it was
/// installed. Also what ships in the repo and what install.sh copies.
const DEFAULT_CONFIG: &str = include_str!("../config.toml");

pub fn load_config(path: &str) -> Config {
    let text = if std::path::Path::new(path).exists() {
        fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("couldn't read config at {path}: {e}");
            std::process::exit(1);
        })
    } else {
        bootstrap_config(path)
    };
    toml::from_str(&text).unwrap_or_else(|e| {
        eprintln!("bad config: {e}");
        std::process::exit(1);
    })
}

/// No config exists yet at `path` — create a starter one instead of
/// crashing. Prefers the RPM-packaged example at /usr/share/sfav so a
/// `dnf install` gets the same starting point as install.sh's git-clone
/// workflow; falls back to the copy embedded in the binary otherwise.
/// Returns the config text either way, even if writing it out failed, so
/// the very first run still works regardless.
fn bootstrap_config(path: &str) -> String {
    let source = fs::read_to_string("/usr/share/sfav/config.toml")
        .unwrap_or_else(|_| DEFAULT_CONFIG.to_string());

    if let Some(parent) = std::path::Path::new(path).parent() {
        let _ = fs::create_dir_all(parent);
    }
    match fs::write(path, &source) {
        Ok(()) => eprintln!("sfav: no config found — created a starter one at {path}"),
        Err(e) => eprintln!(
            "sfav: no config at {path} and couldn't create one ({e}) \u{2014} using built-in defaults for this run"
        ),
    }
    source
}

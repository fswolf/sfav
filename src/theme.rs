use ratatui::{
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Padding},
};

use crate::config::ThemeConfig;

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
pub struct Theme {
    pub border: Color,
    pub header: Color,
    pub highlight_bg: Color,
    pub highlight_fg: Color,
}

impl Theme {
    pub fn from_config(cfg: &ThemeConfig) -> Self {
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
    pub fn block(&self) -> Block<'static> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(self.border))
            .padding(Padding::horizontal(1))
    }
}

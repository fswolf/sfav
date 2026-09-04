# sfav

`ssfav`, but for arbitrary shell commands. Reads a TOML config of
`name` / `command` / `notes`, shows them in a fuzzy-filterable table, and on
Enter execs the chosen command through `$SHELL -c` — replacing the `sfav`
process itself, same trick `ssfav` uses for `ssh`. So it inherits your
terminal directly, no wrapper shell hanging around, and when the command
exits you're back at your prompt (not back inside the launcher).

## Build

```
cargo build --release
cp target/release/sfav ~/.local/bin/
```

Or, to hand this to someone else: `./install.sh` builds it, installs the
binary to `~/.local/bin/sfav`, and drops the example config into
`~/.config/sfav/config.toml` **only if that file doesn't already exist** —
safe to re-run after a `git pull` without clobbering someone's edits.

Compiled and smoke-tested in a sandboxed Ubuntu 24.04 container (rustc
1.75) — the included `Cargo.lock` pins dependency versions known to build
cleanly. On Fedora 44 your rustc will be newer, so `cargo build` will
happily re-resolve to newer patch versions too; the lockfile is just a
known-good fallback.

## Config

Default path: `$XDG_CONFIG_HOME/sfav/config.toml` if that's set, otherwise
`~/.config/sfav/config.toml` — resolved per-user at runtime from the
`$HOME`/`$XDG_CONFIG_HOME` env vars, so this works unmodified for anyone
who installs it, not just you. Or pass a path explicitly:

```
sfav ~/dotfiles/commands.toml
```

Format:

```toml
[[entries]]
name = "rice reload"
command = "hyprctl reload"
notes = "reload hyprland config"
```

`notes` is optional. `command` is passed to `$SHELL -c`, so pipes, `&&`,
env vars, aliases (if your shell's rc file defines them and gets sourced)
all work as typed.

Optional `[theme]` table, hex colors, all optional (defaults shown):

```toml
[theme]
border = "#b48cff"
header = "#b48cff"
highlight_bg = "#463764"
highlight_fg = "#ffffff"
```

`border`/`header` color the box outlines and column headers; `highlight_bg`/
`highlight_fg` color the selected row. Bad hex falls back to white with a
warning printed to stderr instead of crashing.

## Keys

- Type to fuzzy-filter across name/command/notes (plain substring match,
  case-insensitive — swap in `nucleo` or `fuzzy-matcher` if you want real
  fuzzy scoring)
- ↑ / ↓ — move selection
- Enter — exec selected command
- Esc — quit, no-op

## Ideas if you keep going

- Swap the substring filter for `nucleo` (what `ssfav`/helix use) for real
  fuzzy ranking instead of plain `contains`
- Per-entry `confirm = true` for anything destructive (update commands etc.)
  before it execs
- Launch in a new kitty window/tab instead of exec-replacing the current
  one, for commands you want to watch while keeping the picker open
- Waybar/rofi mode: `--dmenu`-style flag that just prints the chosen
  command to stdout instead of exec'ing it
# sfav

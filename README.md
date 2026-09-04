# sfav

**`sfav` is a terminal command launcher for arbitrary shell commands.**

It works similarly to [`ssfav`](https://github.com/...) — instead of managing SSH hosts, `sfav` lets you keep a list of frequently used shell commands in a TOML file and launch them from a fast, keyboard-driven terminal interface.

Search your commands, select one, press **Enter**, and `sfav` hands it directly to your shell.

No wrapper shell. No extra process sitting around. When the command exits, you're back at your normal prompt.

---

## ✨ Features

* 🚀 Fast terminal-based command launcher
* 🔎 Search commands by **name, command, or notes**
* ⌨️ Fully keyboard-driven
* 🐚 Executes commands through your `$SHELL`
* 🔗 Supports normal shell syntax such as pipes, `&&`, redirects, and environment variables
* 🎨 Customizable colors
* ⚙️ Simple TOML configuration
* 📁 Uses standard `$XDG_CONFIG_HOME` configuration paths
* 🔄 Safe installer that won't overwrite an existing configuration
* 🧩 No daemon or background process

---

## 📸 What it does

Instead of remembering commands like:

```bash
hyprctl reload
systemctl --user restart something
docker compose up -d
git pull && cargo build --release
```

you can keep them in `sfav`:

```toml
[[entries]]
name = "rice reload"
command = "hyprctl reload"
notes = "reload hyprland config"

[[entries]]
name = "restart service"
command = "systemctl --user restart something"
notes = "restart my service"

[[entries]]
name = "update project"
command = "git pull && cargo build --release"
notes = "pull and rebuild"
```

Then launch:

```bash
sfav
```

Type to search, select a command, and press **Enter**.

---

## 📦 Installation

### Automatic installation

The easiest way to install `sfav` is with the included installer:

```bash
./install.sh
```

The installer:

1. Builds the release binary with Cargo.
2. Installs it to:

```text
~/.local/bin/sfav
```

3. Installs the example configuration to:

```text
~/.config/sfav/config.toml
```

The configuration file is **only created if it doesn't already exist**, so running the installer again will not overwrite your existing commands.

This also makes it safe to run after pulling updates from Git:

```bash
git pull
./install.sh
```

Make sure `~/.local/bin` is in your `$PATH`.

### Manual installation

If you already have Rust and Cargo installed:

```bash
cargo build --release
cp target/release/sfav ~/.local/bin/
```

---

## ⚙️ Configuration

By default, `sfav` looks for:

```text
$XDG_CONFIG_HOME/sfav/config.toml
```

If `$XDG_CONFIG_HOME` isn't set, it falls back to:

```text
~/.config/sfav/config.toml
```

The path is resolved at runtime using the current user's `$HOME` and `$XDG_CONFIG_HOME`, so the same binary and configuration layout works across different users and systems.

You can also specify a configuration file manually:

```bash
sfav ~/dotfiles/commands.toml
```

---

## 📝 Config format

Commands are defined using TOML:

```toml
[[entries]]
name = "rice reload"
command = "hyprctl reload"
notes = "reload hyprland config"
```

### Fields

| Field     | Required | Description                       |
| --------- | -------- | --------------------------------- |
| `name`    | Yes      | Display name for the command      |
| `command` | Yes      | Shell command to execute          |
| `notes`   | No       | Additional searchable information |

`notes` is optional and can be useful for remembering what a command does.

---

## 🐚 Shell commands

Commands are executed through:

```text
$SHELL -c
```

This means you can use normal shell features, including:

```bash
command1 && command2
```

```bash
command1 | command2
```

```bash
VAR=value command
```

and other shell syntax supported by your shell.

Because `sfav` replaces itself with the command process, the command runs directly in your terminal rather than inside a persistent launcher shell.

When the command finishes, you're returned directly to your normal shell prompt.

> **Note:** Shell aliases and functions depend on how your shell is configured and whether they are available when the command is invoked.

---

## 🎨 Themes

The interface can be customized with an optional `[theme]` section.

All values are optional and have sensible defaults:

```toml
[theme]
border = "#b48cff"
header = "#b48cff"
highlight_bg = "#463764"
highlight_fg = "#ffffff"
```

### Theme options

| Option         | Description                          |
| -------------- | ------------------------------------ |
| `border`       | Color of the interface borders       |
| `header`       | Color of the column headers          |
| `highlight_bg` | Background color of the selected row |
| `highlight_fg` | Text color of the selected row       |

Colors use hexadecimal notation.

If an invalid color is supplied, `sfav` falls back to white and prints a warning to `stderr` rather than crashing.

---

## ⌨️ Keybindings

| Key       | Action                   |
| --------- | ------------------------ |
| `↑` / `↓` | Move selection           |
| `Enter`   | Execute selected command |
| `Esc`     | Quit                     |
| Typing    | Filter commands          |

Search currently performs a **case-insensitive substring match** across:

* Command name
* Command
* Notes

For example, searching for:

```text
docker
```

will find entries containing `docker` anywhere in those fields.

---
 
## 🔨 Building from source

`sfav` is written in Rust.

Build a release version with:

```bash
cargo build --release
```

The resulting binary will be:

```text
target/release/sfav
```

The repository includes a `Cargo.lock` file with dependency versions that have been tested to build successfully.

Newer Rust toolchains may resolve compatible newer patch versions when rebuilding.

---

## 🧪 Compatibility

`sfav` is designed for Linux systems with:

* A POSIX-compatible terminal
* A shell available through `$SHELL`
* Rust/Cargo when building from source

The project has been compiled and smoke-tested in an Ubuntu 24.04 environment.

---

## 📄 License

See [LICENSE](LICENSE) for details.

---

## 💡 Why `sfav`?

There are plenty of ways to create aliases and shell functions.

`sfav` is for the commands you **don't use often enough to memorize, but use often enough that typing them every time is annoying.**

Instead of maintaining a giant `.bashrc` full of aliases, put your commands in one searchable list and let `sfav` find them.

```text
Remember less.
Type less.
Do more.
```

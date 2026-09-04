#!/usr/bin/env bash
set -euo pipefail

REPO_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BIN_DIR="$HOME/.local/bin"
CONFIG_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/sfav"

echo "Building sfav (release)..."
cargo build --release --manifest-path "$REPO_DIR/Cargo.toml"

mkdir -p "$BIN_DIR"
cp "$REPO_DIR/target/release/sfav" "$BIN_DIR/sfav"
echo "Installed binary -> $BIN_DIR/sfav"

mkdir -p "$CONFIG_DIR"
if [ -f "$CONFIG_DIR/config.toml" ]; then
    echo "Existing config found at $CONFIG_DIR/config.toml, leaving it alone."
else
    cp "$REPO_DIR/config.toml" "$CONFIG_DIR/config.toml"
    chmod 600 "$CONFIG_DIR/config.toml"
    echo "Installed example config -> $CONFIG_DIR/config.toml"
fi

case ":$PATH:" in
    *":$BIN_DIR:"*) ;;
    *)
        echo
        echo "Note: $BIN_DIR isn't on your PATH yet. Add this to your shell rc:"
        echo "  export PATH=\"\$PATH:$BIN_DIR\""
        ;;
esac

echo
echo "Done. Edit $CONFIG_DIR/config.toml, then run: sfav"

Name:           sfav
Version:        0.2.0
Release:        1%{?dist}
Summary:        A minimal TUI launcher for shell commands and scripts

License:        MIT
URL:            https://github.com/fswolf/sfav
Source0:        https://github.com/fswolf/sfav/archive/refs/tags/v%{version}/%{name}-%{version}.tar.gz
# Vendored crate sources, since the COPR/mock build root has no network
# access. Regenerate with: cargo vendor vendor && tar -C . -cJf vendor.tar.xz vendor
Source1:        vendor.tar.xz

BuildRequires:  rust
BuildRequires:  cargo

%description
sfav is a terminal command launcher for arbitrary shell commands, inspired
by sshs. Keep frequently used shell commands in a TOML file and launch them
from a fast, keyboard-driven, themeable terminal interface.

%prep
%autosetup
tar -xf %{SOURCE1}
mkdir -p .cargo
cat > .cargo/config.toml <<'EOF'
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

%build
cargo build --release --offline

%install
install -Dm755 target/release/%{name} %{buildroot}%{_bindir}/%{name}
install -Dm644 config.toml %{buildroot}%{_datadir}/%{name}/config.toml

%files
%license LICENSE
%doc README.md
%{_bindir}/%{name}
%{_datadir}/%{name}/config.toml

%changelog
* Thu Sep 03 2026 fswolf - 0.2.0-1
- Initial package

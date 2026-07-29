#!/usr/bin/env bash

set -euo pipefail

RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[1;36m'; YELLOW='\033[0;33m'; RESET='\033[0m'
trap 'echo -e "\n${RED}✗ install.sh failed at line ${LINENO}: ${BASH_COMMAND}${RESET}" >&2' ERR

log()  { echo -e "${CYAN}==>${RESET} $*"; }
ok()   { echo -e "${GREEN}✓${RESET} $*"; }
warn() { echo -e "${YELLOW}!${RESET} $*"; }
die()  { echo -e "${RED}✗ $*${RESET}" >&2; exit 1; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLI_DIR="$SCRIPT_DIR/src/catppuccinifier-cli"
GUI_DIR="$SCRIPT_DIR/src/catppuccinifier-gui"
GUI_TAURI_DIR="$GUI_DIR/src-tauri"
ASSETS_DIR="$SCRIPT_DIR/src/assets"

[[ -d "$SCRIPT_DIR/src" ]] || die "Run this from the root of the extracted source (couldn't find ./src here)."

INSTALL_MODE="system"
SKIP_DEPS=0
SKIP_DESKTOP=0

usage() {
    cat <<EOF
Usage: ./install.sh [options]

  --user         Install to \$HOME/.local instead of /usr (no sudo required)
  --skip-deps    Don't run pacman for system dependencies (assume already installed)
  --no-desktop   Skip installing the icon + .desktop entry (binaries only)
  -h, --help     Show this help
EOF
}

for arg in "$@"; do
    case "$arg" in
        --user) INSTALL_MODE="user" ;;
        --skip-deps) SKIP_DEPS=1 ;;
        --no-desktop) SKIP_DESKTOP=1 ;;
        -h|--help) usage; exit 0 ;;
        *) die "Unknown option: $arg (see --help)" ;;
    esac
done

if [[ "$INSTALL_MODE" == "system" ]]; then
    BIN_DIR="/usr/bin"
    ICON_DIR="/usr/share/pixmaps"
    DESKTOP_DIR="/usr/share/applications"
    SUDO="sudo"
else
    BIN_DIR="$HOME/.local/bin"
    ICON_DIR="$HOME/.local/share/pixmaps"
    DESKTOP_DIR="$HOME/.local/share/applications"
    SUDO=""
fi

install_system_deps() {
    if (( SKIP_DEPS )); then
        warn "Skipping system dependency install (--skip-deps) — assuming build tools + libs are already present"
        return
    fi

    if ! command -v pacman >/dev/null 2>&1; then
        warn "pacman not found — this script targets Arch/CachyOS, so it can't auto-install for you."
        warn "You'll need, at minimum: webkit2gtk-4.1 (equiv: libwebkit2gtk-4.1-dev on Debian/Ubuntu,"
        warn "webkit2gtk4.1-devel on Fedora), a Rust toolchain (rustup.rs), and Node.js + Yarn Classic (v1)."
        warn "Install those yourself, then re-run with --skip-deps."
        return
    fi

    local pkgs=(
        webkit2gtk-4.1
        base-devel
        curl
        wget
        file
        openssl
        appmenu-gtk-module
        libappindicator-gtk3
        librsvg
        xdotool
        desktop-file-utils
        git
        nodejs
        yarn
    )

    if ! command -v cargo >/dev/null 2>&1; then
        log "No Rust toolchain found — adding 'rust' to the install list"
        pkgs+=(rust)
    fi

    log "Installing system + build dependencies (note: webkit2gtk-4.1, NOT webkit2gtk)"
    sudo pacman -S --needed "${pkgs[@]}"
    ok "System + build dependencies present"
}

check_rust() {
    if ! command -v cargo >/dev/null 2>&1; then
        die "cargo still not found. Re-run without --skip-deps to let this script install it, or install Rust yourself (rustup.rs) first."
    fi
    log "Using $(rustc --version)"
}

check_yarn() {
    command -v yarn >/dev/null 2>&1 || die "yarn not found. Re-run without --skip-deps to let this script install it, or install Yarn Classic (v1) yourself first."
    local yarn_major
    yarn_major="$(yarn --version | cut -d. -f1)"
    [[ "$yarn_major" == "1" ]] || die "System yarn is v$(yarn --version) (Berry), but yarn.lock here is v1 (Classic). Install Yarn Classic (v1) instead."
    log "Using yarn $(yarn --version)"
}

js_install() {
    yarn install --frozen-lockfile
}

js_tauri_build() {
    yarn tauri build --no-bundle
}

build_gui() {
    log "Building GUI via the Tauri CLI (yarn install, then yarn tauri build)"
    ( cd "$GUI_DIR" && js_install && js_tauri_build )
    ok "GUI compiled: $GUI_TAURI_DIR/target/release/catppuccinifier-gui"
}

build_cli() {
    log "Compiling CLI (Rust, release)"
    ( cd "$CLI_DIR" && cargo build --release )
    ok "CLI compiled: $CLI_DIR/target/release/catppuccinifier-cli"
}

install_binaries() {
    local gui_bin="$GUI_TAURI_DIR/target/release/catppuccinifier-gui"
    local cli_bin="$CLI_DIR/target/release/catppuccinifier-cli"

    [[ -f "$gui_bin" ]] || die "GUI binary missing at $gui_bin — build must have failed silently."
    [[ -f "$cli_bin" ]] || die "CLI binary missing at $cli_bin — build must have failed silently."

    log "Installing binaries to $BIN_DIR"
    $SUDO install -Dm755 "$gui_bin" "$BIN_DIR/catppuccinifier-gui"
    $SUDO install -Dm755 "$cli_bin" "$BIN_DIR/catppuccinifier"
    ok "Installed catppuccinifier-gui and catppuccinifier"
}

install_desktop_integration() {
    if (( SKIP_DESKTOP )); then
        warn "Skipping icon/.desktop install (--no-desktop)"
        return
    fi

    log "Installing icon and desktop entry"
    $SUDO install -Dm644 "$ASSETS_DIR/catppuccinifier.png" "$ICON_DIR/catppuccinifier.png"

    if command -v desktop-file-install >/dev/null 2>&1 && [[ "$INSTALL_MODE" == "system" ]]; then
        $SUDO desktop-file-install -m 644 --dir "$DESKTOP_DIR" "$ASSETS_DIR/Catppuccinifier.desktop"
    else
        $SUDO install -Dm644 "$ASSETS_DIR/Catppuccinifier.desktop" "$DESKTOP_DIR/Catppuccinifier.desktop"
    fi

    if command -v update-desktop-database >/dev/null 2>&1; then
        $SUDO update-desktop-database "$DESKTOP_DIR"
    else
        warn "update-desktop-database not found — desktop entry may not show up until next login"
    fi
    ok "Desktop entry installed to $DESKTOP_DIR"
}

main() {
    log "Catppuccinifier from-source install ($INSTALL_MODE mode)"
    if [[ "$INSTALL_MODE" == "system" || "$SKIP_DEPS" -eq 0 ]]; then
        log "Caching sudo credentials"
        sudo -v
    fi

    install_system_deps
    check_rust
    check_yarn

    build_gui
    build_cli

    install_binaries
    install_desktop_integration

    echo
    ok "Done. Run 'catppuccinifier-gui' or 'catppuccinifier' (CLI) to launch."
    if [[ "$INSTALL_MODE" == "user" ]]; then
        [[ ":$PATH:" == *":$HOME/.local/bin:"* ]] || warn "$HOME/.local/bin isn't on your PATH yet — add it to your shell rc."
    fi
}

main

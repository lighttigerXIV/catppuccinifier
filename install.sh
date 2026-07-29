#!/usr/bin/env bash
#
# install.sh — Build and install Catppuccinifier (CLI + GUI) from source on Arch/CachyOS.
#
# WHY THIS EXISTS
# ----------------
# The upstream INSTALL.md (and the catppuccinifier-gui-bin/-cli-bin AUR packages,
# which appear to have copied it) list `webkit2gtk` as an Arch dependency. That's
# the legacy WebKit2GTK 4.0/soup2 build, which Arch dropped from the official repos
# — it now only exists as an AUR from-source package, and building it is the reason
# `catppuccinifier-gui-bin` takes forever to "install".
#
# But this source tree is already on Tauri 2 (see src-tauri/Cargo.toml: tauri = "2"),
# and Tauri 2's Linux backend (wry 0.52 in Cargo.lock here) links against
# webkit2gtk-4.1/soup3, not the old 4.0/soup2 build — confirmed by Cargo.lock, which
# only resolves soup3/soup3-sys and webkit2gtk-rs 2.0.1 (the 4.1-targeting release),
# with zero soup2 anywhere in the tree. So no application code changes are needed;
# `webkit2gtk-4.1` is already a prebuilt binary package in Arch's [extra] repo.
# The INSTALL.md dependency list is just stale, and so is `libvips` — grepping every
# Cargo.lock in this tree shows catppuccinifier-rs does its image work with the pure-Rust
# `image` + `lutgen` crates, never touches libvips.
#
# This script installs the correct runtime deps AND whatever build tooling is
# missing (Rust via pacman if no cargo is on PATH, Node.js if needed), builds
# both binaries in release mode, and installs them + the desktop entry —
# mirroring what src/install-cli actually does on a successful release install.
#
# Run this from the root of the extracted source (next to src/, README.md).

set -euo pipefail

# --- keep failures loud, not silent -----------------------------------------
# (set -e + suppressed output is how build scripts hide their real error; don't
# redirect command output to /dev/null anywhere below — let it print.)
RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[1;36m'; YELLOW='\033[0;33m'; RESET='\033[0m'
trap 'echo -e "\n${RED}✗ install.sh failed at line ${LINENO}: ${BASH_COMMAND}${RESET}" >&2' ERR

log()  { echo -e "${CYAN}==>${RESET} $*"; }
ok()   { echo -e "${GREEN}✓${RESET} $*"; }
warn() { echo -e "${YELLOW}!${RESET} $*"; }
die()  { echo -e "${RED}✗ $*${RESET}" >&2; exit 1; }

# --- paths -------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLI_DIR="$SCRIPT_DIR/src/catppuccinifier-cli"
GUI_DIR="$SCRIPT_DIR/src/catppuccinifier-gui"
GUI_TAURI_DIR="$GUI_DIR/src-tauri"
ASSETS_DIR="$SCRIPT_DIR/src/assets"

[[ -d "$SCRIPT_DIR/src" ]] || die "Run this from the root of the extracted source (couldn't find ./src here)."

# --- options -------------------------------------------------------------------
INSTALL_MODE="system"   # system (/usr, needs sudo) or user (~/.local, no sudo)
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

# --- 1. system + build dependencies --------------------------------------------
# Covers both runtime libs (webkit2gtk-4.1 & friends) AND build tooling (rust,
# node) — installed only if actually missing, so an existing rustup toolchain
# or nvm-managed node is left alone rather than fought with.
install_system_deps() {
    if (( SKIP_DEPS )); then
        warn "Skipping system dependency install (--skip-deps) — assuming build tools + libs are already present"
        return
    fi

    if ! command -v pacman >/dev/null 2>&1; then
        warn "pacman not found — this script targets Arch/CachyOS, so it can't auto-install for you."
        warn "You'll need, at minimum: webkit2gtk-4.1 (equiv: libwebkit2gtk-4.1-dev on Debian/Ubuntu,"
        warn "webkit2gtk4.1-devel on Fedora), a Rust toolchain (rustup.rs), and Node.js + yarn/npm."
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
        npm
    )

    # Only pull in pacman's `rust` if there's no cargo on PATH at all — if you're
    # managing your toolchain with rustup, this leaves it alone instead of
    # installing a second, conflicting cargo/rustc.
    if ! command -v cargo >/dev/null 2>&1; then
        log "No Rust toolchain found — adding 'rust' to the install list"
        pkgs+=(rust)
    fi

    log "Installing system + build dependencies (note: webkit2gtk-4.1, NOT webkit2gtk)"
    sudo pacman -S --needed "${pkgs[@]}"
    ok "System + build dependencies present"
}

# --- 2. toolchain checks -------------------------------------------------------
# By this point install_system_deps has already tried to fix a missing
# toolchain via pacman (unless --skip-deps was passed) — these just confirm
# it actually worked and fail loudly with a clear next step if not.
check_rust() {
    if ! command -v cargo >/dev/null 2>&1; then
        die "cargo still not found. Re-run without --skip-deps to let this script install it, or install Rust yourself (rustup.rs) first."
    fi
    log "Using $(rustc --version)"
}

JS_PKG_MANAGER=""
check_js_pkg_manager() {
    # This project's yarn.lock is Yarn Classic (v1) format. If the system `yarn`
    # resolves to Yarn Berry (v2+, common via corepack defaults) it won't install
    # from a v1 lockfile predictably, so only use it when it's actually v1.
    if command -v yarn >/dev/null 2>&1; then
        local yarn_major
        yarn_major="$(yarn --version | cut -d. -f1)"
        if [[ "$yarn_major" == "1" ]]; then
            JS_PKG_MANAGER="yarn"
        else
            warn "System yarn is v$(yarn --version) (Berry), but yarn.lock here is v1 (Classic) — using npm instead"
            JS_PKG_MANAGER="npm"
        fi
    fi

    if [[ -z "$JS_PKG_MANAGER" ]]; then
        command -v npm >/dev/null 2>&1 || die "npm still not found. Re-run without --skip-deps to let this script install Node.js via pacman, or install it yourself first."
        JS_PKG_MANAGER="npm"
    fi
    log "Using $JS_PKG_MANAGER for the frontend build"
}

js_install() {
    if [[ "$JS_PKG_MANAGER" == "yarn" ]]; then
        yarn install --frozen-lockfile
    else
        npm install
    fi
}

js_build() {
    if [[ "$JS_PKG_MANAGER" == "yarn" ]]; then
        yarn build
    else
        npm run build
    fi
}

# --- 3. build -------------------------------------------------------------------
build_gui() {
    log "Building GUI frontend (SvelteKit -> $GUI_DIR/build)"
    ( cd "$GUI_DIR" && js_install && js_build )
    ok "Frontend built"

    log "Compiling GUI backend (Rust/Tauri, release) — this links against webkit2gtk-4.1"
    ( cd "$GUI_TAURI_DIR" && cargo build --release )
    ok "GUI compiled: $GUI_TAURI_DIR/target/release/catppuccinifier-gui"
}

build_cli() {
    log "Compiling CLI (Rust, release)"
    ( cd "$CLI_DIR" && cargo build --release )
    ok "CLI compiled: $CLI_DIR/target/release/catppuccinifier-cli"
}

# --- 4. install -------------------------------------------------------------------
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

# --- main -------------------------------------------------------------------------
main() {
    log "Catppuccinifier from-source install ($INSTALL_MODE mode)"
    # Need root for /usr installs (system mode) and/or for pacman (unless deps
    # are being skipped) — cache credentials once up front instead of prompting
    # mid-build.
    if [[ "$INSTALL_MODE" == "system" || "$SKIP_DEPS" -eq 0 ]]; then
        log "Caching sudo credentials"
        sudo -v
    fi

    install_system_deps
    check_rust
    check_js_pkg_manager

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

#!/usr/bin/env bash

set -Eeuo pipefail

RED='\033[0;31m'; GREEN='\033[0;32m'; CYAN='\033[1;36m'; YELLOW='\033[0;33m'; RESET='\033[0m'
trap 'echo -e "\n${RED}✗ install.sh failed at line ${LINENO}: ${BASH_COMMAND}${RESET}" >&2' ERR

log()  { echo -e "${CYAN}==>${RESET} $*"; }
ok()   { echo -e "${GREEN}✓${RESET} $*"; }
warn() { echo -e "${YELLOW}!${RESET} $*"; }
die()  { echo -e "${RED}✗ $*${RESET}" >&2; exit 1; }
have() { command -v "$1" >/dev/null 2>&1; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CLI_DIR="$SCRIPT_DIR/src/catppuccinifier-cli"
GUI_DIR="$SCRIPT_DIR/src/catppuccinifier-gui"
GUI_TAURI_DIR="$GUI_DIR/src-tauri"
ASSETS_DIR="$SCRIPT_DIR/src/assets"

[[ -d "$SCRIPT_DIR/src" ]] || die "Run this from the root of the extracted source (couldn't find ./src here)."

# Cargo.lock pins crates (icu_* 2.0, zerovec 0.11, time 0.3.41) that need Rust >= 1.82.
MIN_RUST="1.82.0"
# Vite 6 / SvelteKit 2 need Node 18+.
MIN_NODE_MAJOR=18
OS_RELEASE="${OS_RELEASE:-/etc/os-release}"

INSTALL_MODE="system"
SKIP_DEPS=0
SKIP_DESKTOP=0
FORCE_DISTRO=""
DISTRO=""

usage() {
    cat <<EOF
Usage: ./install.sh [options]

  --user            Install to \$HOME/.local instead of /usr (no sudo needed for the install itself)
  --skip-deps       Don't install system dependencies (assume already installed)
  --no-desktop      Skip installing the icon + .desktop entry (binaries only)
  --distro FAMILY   Force the distro family: arch | debian | fedora
                    (normally auto-detected from /etc/os-release)
  -h, --help        Show this help

Supported families (and derivatives): Arch/CachyOS/Manjaro/EndeavourOS, Debian/Ubuntu/Mint/Pop!_OS,
Fedora/RHEL-likes/Nobara.
EOF
}

while (( $# )); do
    case "$1" in
        --user) INSTALL_MODE="user" ;;
        --skip-deps) SKIP_DEPS=1 ;;
        --no-desktop) SKIP_DESKTOP=1 ;;
        --distro)
            shift
            [[ $# -gt 0 ]] || die "--distro needs a value: arch | debian | fedora"
            FORCE_DISTRO="$1"
            ;;
        --distro=*) FORCE_DISTRO="${1#--distro=}" ;;
        -h|--help) usage; exit 0 ;;
        *) die "Unknown option: $1 (see --help)" ;;
    esac
    shift
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
    export PATH="$HOME/.local/bin:$PATH"
fi

# Package-manager commands need root regardless of install mode.
if (( EUID == 0 )); then
    PKG_SUDO=""
    [[ "$INSTALL_MODE" == "system" ]] && SUDO=""
else
    PKG_SUDO="sudo"
fi

# ---------------------------------------------------------------------------
# Distro detection
# ---------------------------------------------------------------------------
detect_distro() {
    if [[ -n "$FORCE_DISTRO" ]]; then
        case "$FORCE_DISTRO" in
            arch|debian|fedora) DISTRO="$FORCE_DISTRO" ;;
            *) die "Unknown --distro '$FORCE_DISTRO' (expected: arch | debian | fedora)" ;;
        esac
        return
    fi

    local id="" like=""
    if [[ -r "$OS_RELEASE" ]]; then
        id="$(. "$OS_RELEASE"; echo "${ID:-}")"
        like="$(. "$OS_RELEASE"; echo "${ID_LIKE:-}")"
    fi
    local all=" $id $like "

    case "$all" in
        *" arch "*|*" cachyos "*|*" manjaro "*|*" endeavouros "*) DISTRO="arch" ;;
        *" debian "*|*" ubuntu "*)                                 DISTRO="debian" ;;
        *" fedora "*|*" rhel "*|*" centos "*)                      DISTRO="fedora" ;;
    esac

    # Fallback: go by whichever package manager is present
    if [[ -z "$DISTRO" ]]; then
        if   have pacman;  then DISTRO="arch"
        elif have apt-get; then DISTRO="debian"
        elif have dnf;     then DISTRO="fedora"
        fi
    fi

    if [[ -z "$DISTRO" ]] && (( ! SKIP_DEPS )); then
        die "Couldn't detect your distro (ID='${id}', ID_LIKE='${like}'). Re-run with --distro arch|debian|fedora, or install deps yourself and use --skip-deps."
    fi
}

# ---------------------------------------------------------------------------
# Dependencies
# ---------------------------------------------------------------------------
install_deps_arch() {
    have pacman || die "Detected Arch family but pacman isn't available."

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

    # Don't pull in 'rust' if cargo or rustup is already around (they conflict)
    if ! have cargo && ! have rustup; then
        log "No Rust toolchain found — adding 'rust' to the install list"
        pkgs+=(rust)
    fi

    # Don't fight an existing nodejs-lts-* / nvm / yarn install
    have node && pkgs=("${pkgs[@]/nodejs}")
    have yarn && pkgs=("${pkgs[@]/yarn}")
    local filtered=() p
    for p in "${pkgs[@]}"; do [[ -n "$p" ]] && filtered+=("$p"); done
    pkgs=("${filtered[@]}")

    log "Installing system + build dependencies via pacman (note: webkit2gtk-4.1, NOT webkit2gtk)"
    $PKG_SUDO pacman -S --needed "${pkgs[@]}"
}

install_deps_debian() {
    have apt-get || die "Detected Debian family but apt-get isn't available."

    local pkgs=(
        build-essential
        pkg-config
        libwebkit2gtk-4.1-dev
        libssl-dev
        libxdo-dev
        libayatana-appindicator3-dev
        librsvg2-dev
        curl
        wget
        file
        desktop-file-utils
        git
        nodejs
        npm
    )

    # Existing node/npm (nvm, NodeSource, ...) are left alone
    have node && pkgs=("${pkgs[@]/nodejs}")
    { have node || have npm; } && pkgs=("${pkgs[@]/npm}")
    local filtered=() p
    for p in "${pkgs[@]}"; do [[ -n "$p" ]] && filtered+=("$p"); done
    pkgs=("${filtered[@]}")

    log "Installing system + build dependencies via apt"
    $PKG_SUDO apt-get update
    if ! apt-cache show libwebkit2gtk-4.1-dev >/dev/null 2>&1; then
        die "libwebkit2gtk-4.1-dev isn't available in your apt repos (needs Debian 12+/Ubuntu 22.04+). Tauri v2 can't be built on this release."
    fi
    $PKG_SUDO apt-get install -y "${pkgs[@]}"
    # Rust is handled by ensure_rust (distro rustc is often too old for Tauri v2);
    # Yarn Classic is handled by ensure_yarn.
}

install_deps_fedora() {
    have dnf || die "Detected Fedora family but dnf isn't available."

    if [[ -e /run/ostree-booted ]]; then
        warn "This looks like an immutable (rpm-ostree) system. dnf installs won't work here —"
        warn "build inside a toolbox/distrobox container, or install deps yourself and use --skip-deps."
    fi

    local pkgs=(
        gcc
        gcc-c++
        make
        pkgconf-pkg-config
        webkit2gtk4.1-devel
        openssl-devel
        libxdo-devel
        libappindicator-gtk3-devel
        librsvg2-devel
        curl
        wget
        file
        desktop-file-utils
        git
        nodejs
        npm
    )

    if ! have cargo && ! have rustup; then
        log "No Rust toolchain found — adding 'rust' and 'cargo' to the install list"
        pkgs+=(rust cargo)
    fi

    have node && pkgs=("${pkgs[@]/nodejs}")
    { have node || have npm; } && pkgs=("${pkgs[@]/npm}")
    local filtered=() p
    for p in "${pkgs[@]}"; do [[ -n "$p" ]] && filtered+=("$p"); done
    pkgs=("${filtered[@]}")

    log "Installing system + build dependencies via dnf"
    $PKG_SUDO dnf install -y "${pkgs[@]}"
}

install_system_deps() {
    if (( SKIP_DEPS )); then
        warn "Skipping system dependency install (--skip-deps) — assuming build tools + libs are already present"
        return
    fi

    case "$DISTRO" in
        arch)   install_deps_arch ;;
        debian) install_deps_debian ;;
        fedora) install_deps_fedora ;;
        *)      die "No dependency installer for distro family '$DISTRO'." ;;
    esac
    ok "System + build dependencies present"
}

# ---------------------------------------------------------------------------
# Toolchain checks (Rust, Node, Yarn Classic)
# ---------------------------------------------------------------------------
version_ge() {  # version_ge A B  -> true if A >= B
    [[ "$(printf '%s\n%s\n' "$2" "$1" | sort -V | sed -n 1p)" == "$2" ]]
}

rust_ok() {
    have cargo && cargo --version >/dev/null 2>&1 && have rustc || return 1
    local v
    v="$(rustc --version | awk '{print $2}')"
    version_ge "${v%%-*}" "$MIN_RUST"
}

ensure_rust() {
    if [[ -r "$HOME/.cargo/env" ]]; then
        # shellcheck disable=SC1091
        . "$HOME/.cargo/env"
    fi

    if rust_ok; then
        log "Using $(rustc --version)"
        return
    fi

    if (( SKIP_DEPS )); then
        die "Need Rust >= $MIN_RUST (found: $(rustc --version 2>/dev/null || echo none)). Install/update via rustup.rs, or re-run without --skip-deps."
    fi

    if have rustup; then
        log "rustup present but toolchain missing/too old — installing/updating stable"
        rustup toolchain install stable --profile minimal
        rustup default stable
    else
        warn "Rust >= $MIN_RUST not found (found: $(rustc --version 2>/dev/null || echo none)) — installing via rustup"
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal
    fi

    if [[ -r "$HOME/.cargo/env" ]]; then
        # shellcheck disable=SC1091
        . "$HOME/.cargo/env"
    fi
    rust_ok || die "Rust >= $MIN_RUST still not available after install attempt."
    log "Using $(rustc --version)"
}

check_node() {
    have node || die "node not found. Re-run without --skip-deps to let this script install it, or install Node.js >= $MIN_NODE_MAJOR yourself."
    local major
    major="$(node --version | sed 's/^v//' | cut -d. -f1)"
    if (( major < MIN_NODE_MAJOR )); then
        die "Node.js $(node --version) is too old — the frontend (Vite 6 / SvelteKit 2) needs v$MIN_NODE_MAJOR or newer. Install a newer Node via nvm, NodeSource, or your distro's newer nodejs stream, then re-run."
    fi
    log "Using node $(node --version)"
}

ensure_yarn() {
    if have yarn; then return; fi
    if (( SKIP_DEPS )); then return; fi   # check_yarn will report it

    have npm || die "npm not found, so Yarn Classic can't be installed automatically. Install yarn (v1) yourself."
    log "Installing Yarn Classic (v1) via npm"
    if [[ "$INSTALL_MODE" == "user" ]]; then
        npm install -g --prefix "$HOME/.local" yarn@1
    else
        $PKG_SUDO npm install -g yarn@1
    fi
}

check_yarn() {
    have yarn || die "yarn not found. Re-run without --skip-deps to let this script install it, or install Yarn Classic (v1) yourself first."
    local yarn_ver yarn_major
    yarn_ver="$(yarn --version 2>/dev/null || true)"
    yarn_major="${yarn_ver%%.*}"
    [[ "$yarn_major" == "1" ]] || die "The 'yarn' on PATH reports '${yarn_ver:-nothing}', but yarn.lock here is v1 (Classic). Install Yarn Classic (v1) instead (on Debian, the 'yarn' from 'cmdtest' is a different tool — remove it)."
    log "Using yarn $yarn_ver"
}

# ---------------------------------------------------------------------------
# Build
# ---------------------------------------------------------------------------
js_install() {
    yarn install --frozen-lockfile
}

js_tauri_build() {
    yarn tauri build --no-bundle
}

cargo_target_dir() {
    ( cd "$1" && cargo metadata --format-version 1 --no-deps | grep -o '"target_directory":"[^"]*"' | cut -d'"' -f4 )
}

build_gui() {
    log "Building GUI via the Tauri CLI (yarn install, then yarn tauri build)"
    ( cd "$GUI_DIR" && js_install && js_tauri_build )
    ok "GUI compiled: $(cargo_target_dir "$GUI_TAURI_DIR")/release/catppuccinifier-gui"
}

build_cli() {
    log "Compiling CLI (Rust, release)"
    ( cd "$CLI_DIR" && cargo build --release )
    ok "CLI compiled: $(cargo_target_dir "$CLI_DIR")/release/catppuccinifier-cli"
}

# ---------------------------------------------------------------------------
# Install
# ---------------------------------------------------------------------------
install_binaries() {
    local gui_bin cli_bin
    gui_bin="$(cargo_target_dir "$GUI_TAURI_DIR")/release/catppuccinifier-gui"
    cli_bin="$(cargo_target_dir "$CLI_DIR")/release/catppuccinifier-cli"

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

    if have desktop-file-install && [[ "$INSTALL_MODE" == "system" ]]; then
        $SUDO desktop-file-install -m 644 --dir "$DESKTOP_DIR" "$ASSETS_DIR/Catppuccinifier.desktop"
    else
        $SUDO install -Dm644 "$ASSETS_DIR/Catppuccinifier.desktop" "$DESKTOP_DIR/Catppuccinifier.desktop"
    fi

    if have update-desktop-database; then
        $SUDO update-desktop-database "$DESKTOP_DIR"
    else
        warn "update-desktop-database not found — desktop entry may not show up until next login"
    fi
    ok "Desktop entry installed to $DESKTOP_DIR"
}

main() {
    detect_distro
    log "Catppuccinifier from-source install ($INSTALL_MODE mode, distro family: ${DISTRO:-unknown})"

    if [[ -n "$PKG_SUDO$SUDO" ]] && { [[ "$INSTALL_MODE" == "system" ]] || (( ! SKIP_DEPS )); }; then
        log "Caching sudo credentials"
        sudo -v
    fi

    install_system_deps
    ensure_rust
    check_node
    ensure_yarn
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

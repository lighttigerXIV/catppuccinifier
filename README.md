<div align="center">

<img src="./src/assets/catppuccinifier.png" height="128" width="128">
<h1>Catppuccinifier</h1>

</div>

This project gives you a cli and a gui for catppuccinifying your wallpapers. It's available for Linux and Windows. It uses [lutgen](https://github.com/ozwaldorf/lutgen-rs) under the hood.

<h3>GUI</h3>
<div align="center">
<img src="https://github.com/user-attachments/assets/a9c86c19-1c00-4c83-8b43-007086743ce1" width="1000">
</div>

<h3>CLI</h3>
<div align="center">
<img src="https://github.com/user-attachments/assets/86a3ebe8-8281-4120-80cb-c0fc20ff758e" width="1000">
</div>

## Quick Install (Linux, build from source)

```bash
git clone https://github.com/MurderFromMars/catppuccinifier.git && cd catppuccinifier && ./install.sh
```

This clones the repo and runs `install.sh`, which detects your distro, installs any missing system + build dependencies, builds the CLI and GUI in release mode, and installs both to `/usr/bin` (plus the icon and desktop entry).

Supported distro families (and their derivatives):

|Family|Examples|Package manager|
|------|--------|---------------|
|Arch|Arch, CachyOS, Manjaro, EndeavourOS|`pacman`|
|Debian|Debian 12+, Ubuntu 22.04+, Linux Mint, Pop!_OS|`apt`|
|Fedora|Fedora, Nobara, RHEL-likes|`dnf`|

The family is read from `/etc/os-release` (`ID` / `ID_LIKE`), falling back to whichever package manager is available. On any other distro the script stops and asks you to install the dependencies yourself and re-run with `--skip-deps`.

Options:

|Flag|Description|
|----|-----------|
|`--user`|Install to `$HOME/.local` instead of `/usr` (no sudo needed for the install itself)|
|`--skip-deps`|Don't install system dependencies (assume build tools + libs are already installed)|
|`--no-desktop`|Skip installing the icon + `.desktop` entry|
|`--distro FAMILY`|Force the distro family: `arch`, `debian` or `fedora` (normally auto-detected)|
|`-h`, `--help`|Show usage|

<details>
  <summary>What the script installs and checks</summary>

  **System packages**
  - **Arch:** `webkit2gtk-4.1 base-devel curl wget file openssl appmenu-gtk-module libappindicator-gtk3 librsvg xdotool desktop-file-utils git nodejs yarn` (plus `rust` if there is no Rust toolchain)
  - **Debian / Ubuntu:** `build-essential pkg-config libwebkit2gtk-4.1-dev libssl-dev libxdo-dev libayatana-appindicator3-dev librsvg2-dev curl wget file desktop-file-utils git nodejs npm`
  - **Fedora:** `gcc gcc-c++ make pkgconf-pkg-config webkit2gtk4.1-devel openssl-devel libxdo-devel libappindicator-gtk3-devel librsvg2-devel curl wget file desktop-file-utils git nodejs npm` (plus `rust cargo` if there is no Rust toolchain)

  An existing Node.js, npm or Yarn install (nvm, `nodejs-lts-*`, ...) is left alone.

  **Toolchain**
  - **Rust 1.82 or newer.** If it is missing or older (common on Debian/Ubuntu), the script installs the stable toolchain through [rustup](https://rustup.rs).
  - **Node.js 18 or newer.** The script stops with a message if yours is older. Ubuntu 22.04's packaged Node is too old, so use [nvm](https://github.com/nvm-sh/nvm) or NodeSource there.
  - **Yarn Classic (v1).** Installed through npm if missing. Yarn Berry (v2+) is rejected because `yarn.lock` is v1.

</details>

Notes:
- You need internet access during the build: the Rust crates, the npm packages and the `catppuccinifier-rs` git dependency are all fetched at build time.
- Debian 11 and Ubuntu 20.04 are not supported because they don't ship `libwebkit2gtk-4.1-dev`, which the Tauri v2 GUI needs.
- Immutable systems (Silverblue, Kinoite, ...) can't use `dnf` installs directly. Build inside a toolbox/distrobox container, or install the dependencies yourself and use `--skip-deps`.
- With `--user`, make sure `$HOME/.local/bin` is on your `PATH`.

Want the AUR, NixOS, or Windows install? See [INSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/INSTALL.md).

## How to use cli
The cli version works by selecting the flavor, the advanced properties and the image to be generated.

|Short|Full|Possible Values|Description|
------|----|---------------|-----------|
|-f|--flavor|latte frappe macchiato mocha oled all| Selects the flavor (multiple can be used) [Default: all]|
|-i|--images| * | Selects the images to generate from. [Field Required] |
|  |--hald| 2 .. 16 | Selects the hald level|
|-a|--algorithm| shepards-method gaussian-rbf linear-rbf gaussian-sampling nearest-neighbor | Selects the algorithm|
|  |--euclide| 0 .. 512 | Changes euclide value|
|  |--nearest| 2 .. 26 | Changes nearest value|
|  |--mean| 0 .. 255 | Changes mean value|
|  |--std| 0 .. 255 | Changes std value|
|  |--iterations| 0 .. 1024 | Changes iterations value|
|  |--power| 0 .. 4 | Changes power value|
|-h|--help| | Shows Help|
|-V|--version| | Shows app version|

### Examples
    catppuccinifier fuji.jpg

    catppuccinifier wallpaper.png wallpaper.jpg -f mocha macchiato --nearest 3

## Installation
For AUR packages, NixOS, Windows, and manual dependency lists, see [INSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/INSTALL.md). For a from-source install on Arch, Debian/Ubuntu or Fedora, see [Quick Install](#quick-install-linux-build-from-source) above.

## Uninstall
To uninstall catppuccinifier you can use steps presented in the [UNINSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/UNINSTALL.md)

If you installed with `install.sh` above (a from-source build, not a release download), remove it with:

```bash
sudo rm -f /usr/bin/catppuccinifier /usr/bin/catppuccinifier-gui /usr/share/pixmaps/catppuccinifier.png /usr/share/applications/Catppuccinifier.desktop
```

(drop `sudo` and swap `/usr` for `$HOME/.local` if you installed with `--user`.)

## Other amazing people that worked on the project
- [Isabel](https://github.com/isabelroses)
- [Ozwaldorf](https://github.com/ozwaldorf/lutgen-rs)

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

## Quick Install (Arch / CachyOS, build from source)

```bash
git clone https://github.com/MurderFromMars/catppuccinifier.git && cd catppuccinifier && ./install.sh
```

This clones the repo and runs `install.sh`, which installs any missing system + build dependencies via `pacman`, builds the CLI and GUI in release mode, and installs both to `/usr/bin` (plus the icon and desktop entry). Options:

|Flag|Description|
|----|-----------|
|`--user`|Install to `$HOME/.local` instead of `/usr` (no sudo required)|
|`--skip-deps`|Skip the `pacman` step (assume build tools + libs are already installed)|
|`--no-desktop`|Skip installing the icon + `.desktop` entry|
|`-h`, `--help`|Show usage|

Not on Arch/CachyOS, or want the AUR, NixOS, or Windows install? See [INSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/INSTALL.md).

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
For AUR packages, NixOS, Windows, and manual dependency lists, see [INSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/INSTALL.md). For a quick from-source install on Arch/CachyOS, see [Quick Install](#quick-install-arch--cachyos-build-from-source) above.

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

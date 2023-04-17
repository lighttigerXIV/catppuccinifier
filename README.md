<h1 align="center">Catppuccinifier</h1>

This project gives you a cli and a gui for catppuccinifying your wallpapers.

> **Note** <br>
> The cli is avaliable for windows and linux, however the gui is only avaliable for linux

<img src="https://user-images.githubusercontent.com/35658492/229366244-aa61e131-06d1-4f1c-a507-65927cb4cc4a.png" width="600" >

<img src="https://user-images.githubusercontent.com/35658492/232163107-7f8c5d30-1912-4e9d-9a35-1f21b9feef28.png" width="600">

## How to use cli
The cli version works by selecting the flavor, the noise level and the image to be generated.

|Short|Full|Possible Values|Description|
------|----|---------------|-----------|
|-f|--flavor|latte frappe macchiato mocha oled all| Selects the flavor to generate the image (multiple can be used) [Default: all]|
|-n|--noise|0 1 2 3 4| Selects the noise level to generate the image. [Default: 4]|
|-i|--image| * | Selects the image to generate from. [Field Required] |

### Examples
    catppuccinifier -i fuji.jpg

    catppuccinifier -f mocha macchiato -n 3 -i fuji.jpg

## Installation

### arch linux (AUR)
    paru catppuccinifier-cli-git
    paru catppuccinifier-gui-git

### Prerequisites

##### Arch Linux
```bash
sudo pacman -S imagemagick libadwaita
```
##### Debian / Ubuntu
```bash
sudo apt install imagemagick libadwaita-1-0
```
##### Fedora
```bash
sudo dnf install ImageMagick libadwaita
```
##### Windows
In order to install catppuccinifier you need the **image magick** too. It can be download [here](https://imagemagick.org/script/download.php#windows)

### Genral installation (recommended)

#### Download
To install catppuccinifier in your system go to the [releases](https://github.com/lighttigerXIV/catppuccinifier/releases) page, download the correct zip for you OS and extract it.

#### Linux

Cloning the repo and install it:
```bash
git clone https://github.com/lighttigerXIV/catppuccinifier.git
cd catppuccinifier
chmod +x install
chmod +x uninstall
chmod +x installation-files/catppuccinifier
chmod +x installation-files/catppuccinifier-gui
./install
```
#### Windows
On windows you can simply run `install.exe` as administrator.

###### Uninstall
To uninstall, go to the cloned folder and run the uninstall script:
```bash
./uninstall
```

### Building from source

### Linux
```bash
git clone https://githb.com/lighttigerXIV/catppuccinifier.git
cd catppuccinifier/src/Linux/binaries-source/install-cli
cargo build --release
cd target/release
chmod +x install-cli # maynot be needed
./install-cli
```

## Other amazing people that worked on the project
- [Isabel](https://github.com/isabelroses)
- [Gingeh](https://github.com/Gingeh)

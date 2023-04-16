## About
This project gives you a script and a gui for catppuccinifying your wallpapers.

The cli version is available on linux and windows.

The gui version is oly available for linux users at the moment.

<img src="https://user-images.githubusercontent.com/35658492/229366244-aa61e131-06d1-4f1c-a507-65927cb4cc4a.png" width="600" >

<img src="https://user-images.githubusercontent.com/35658492/232163107-7f8c5d30-1912-4e9d-9a35-1f21b9feef28.png" width="600">

## Dependencies

### Arch Linux
    sudo pacman -S imagemagick libadwaita
  
### Debian 
    sudo apt install imagemagick libadwaita-1-0
    
### Windows
In order to install catppuccinifier you need the **image magick** too. It can be download [here](https://imagemagick.org/script/download.php#windows)
    
## Installation
### Download
To install catppuccinifier in your system go to the [releases](https://github.com/lighttigerXIV/catppuccinifier/releases) page, download the correct zip for you OS and extract it.

### Linux
#### Option 1
Cloning the repo and install it:

    git clone https://github.com/lighttigerXIV/catppuccinifier.git
    cd catppuccinifier
    chmod +x install
    chmod +x uninstall
    chmod +x installation-files/catppuccinifier
    chmod +x installation-files/catppuccinifier-gui
    ./install

#### Option 2
**BTW**, if you are a Arch Linux user, there's a AUR package for the cli and the gui.

For the cli you can use:

    paru catppuccinifier-cli-git

For the gui you can use:

    paru catppuccinifier-gui-git
    
### Windows
On windows you can simply run install.exe as administrator.

## Uninstall
To uninstall, go to the cloned folder and run the uninstall script:

    ./uninstall

## How to use cli
The cli version works by selecting the flavor, the noise level and the image to be generated.

|Short|Full|Possible Values|Description|
------|----|---------------|-----------|
|-f|--flavor|latte frappe macchiato mocha oled all| Selects the flavor to generate the image (multiple can be used) [Default: all]|
|-n|--noise|0 1 2 3 4| Selects the noise level to generate the image. [Default: 4]|
|-i|--image| * | Selects the image to generate from. [Field Required] |

### Examples
```catppuccinifier -i fuji.jpg```

```catppuccinifier -f mocha macchiato -n 3 -i fuji.jpg```

## Other amazing people that worked on the project
- [Isabel](https://github.com/isabelroses)
- [Gingeh](https://github.com/Gingeh)

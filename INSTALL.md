Catppuccinfier is available for Linux and Windows

<details>
  <summary>Linux</summary>
  
  ## Arch Linux
  Arch users have the option to install the programs through the AUR.
  
  For the cli tool:
  
    paru catppuccinifier-cli-git
  
  For the gui tool:
  
    paru catppuccinifier-gui-git
  
  For both tools:
  
    paru catppuccinifier-cli-git
    paru catppuccinifier-gui-git
    
  ## General Install
  ### Dependencies
  
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
  
  ### Installation
  - Download Linux version in the [releases](https://github.com/lighttigerXIV/catppuccinifier/releases) page
  - Extract the zip and go inside the folder
  - Run the following:
  
        chmod +x install
        chmod +x uninstall
        chmod +x installation-files/catppuccinifier
        chmod +x installation-files/catppuccinifier-gui
        ./install
  
</details/>

<details>
  <summary>Windows</summary>
  
  ### Dependencies
  
  This program relies on **Image Magick** to work. It can be downloaded [here](https://imagemagick.org/script/download.php#windows)
  
  ### Installation
  - Download Windows version in the [releases](https://github.com/lighttigerXIV/catppuccinifier/releases) page
  - Extract the zip and go inside the folder
  - Run the `install.exe` as administrator 
  
</details/>

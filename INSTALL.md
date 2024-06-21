Catppuccinfier is available for Linux and Windows

<details>
  <summary>Linux</summary>
  
  ## Arch Linux
  Arch users have the option to install the programs through the AUR.

  > [!WARNING]
  > [catppuccinifier-gui-git](https://aur.archlinux.org/packages/catppuccinifier-gui-git) is currenly broken
  
  For the cli tool:
  
    paru catppuccinifier-cli-bin
    # Or
    paru catppuccinifier-cli-git
  
  For the gui tool:
  
    paru catppuccinifier-gui-bin
  
  For both tools:
  
    paru catppuccinifier-bin

  ## NixOS
  Nix users can use the packages `catppuccinifier-gui` and `catppuccinifier-cli` provided by nixpkgs for the latest release.

  You also have the option to install the CLI via a flake input for a more up to date experince.

  ### Flakes-Enabled Nix
  For a flakes-enabled system, add this repo as a flake input:
  ```nix
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";

    catppuccinifier = {
      url = "github:lighttigerXIV/catppuccinifier";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  ```

  If you're using `home-manager`:
  ```nix
  outputs = { self, nixpkgs, home-manager, catppuccinifier, ... }@inputs:
    # ...
      nixosConfigurations.nixos = nixpkgs.lib.nixosSystem {
        # ...
        modules = [ 
          ./config/configuration.nix
          home-manager.nixosModules.home-manager
          ({config, ...}:{
            # ...
            home-manager.extraSpecialArgs = {
              inherit inputs;
            # ...
            };
          })
          # ...
      };
    };
  ```

  And then add it to your packages with `catppuccinifier.packages.${pkgs.system}.cli`.
    
  ## General Install
  ### Dependencies
  
  ##### Arch Linux
  ```bash 
  sudo pacman -S libadwaita webkit2gtk base-devel curl wget openssl appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips
  ```
  ##### Debian / Ubuntu
  ```bash
  sudo apt install libadwaita-1-0 libwebkit2gtk-4.0-dev build-essential curl wget libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
  ```
  ##### Fedora
  ```bash
  sudo dnf install libadwaita webkit2gtk4.0-devel openssl-devel curl wget libappindicator-gtk3 librsvg2-devel
  ```
  
  ### Installation
  - Download Linux version in the [releases](https://github.com/lighttigerXIV/catppuccinifier/releases) page
  - Extract the zip and go inside the folder
  - Run the following:
  ```bash
  chmod +x ./install
  chmod +x ./uninstall
  chmod +x ./installation-files/catppuccinifier
  chmod +x ./installation-files/catppuccinifier-gui
  ./install
  ```
</details/>

<details>
  <summary>Windows</summary>
  
  ### Installation
  - Download Windows version in the [releases](https://github.com/lighttigerXIV/catppuccinifier/releases) page
  - Extract the zip and go inside the folder
  - Run the `install.exe` as administrator 
  
</details/>

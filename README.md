## About
This project gives you a script and a gui for catppuccinifying your wallpapers.
For now it's only availabe on **linux**.


<img src="https://user-images.githubusercontent.com/35658492/227786042-a2d272df-bf90-4d66-8e4f-9eb2a0c0d91a.png" height="600" >

<img src="https://user-images.githubusercontent.com/35658492/227786196-26910de7-37b2-4646-a43e-3f6a30f4c447.png" height="600">





## Dependencies

### Arch Linux
    sudo pacman -S imagemagick
  
### Debian 
    sudo apt install imagemagick
    
## Install
### Locally
    git clone https://github.com/lighttigerXIV/catppuccinifier.git
    cd catppuccinifier
    chmod +x ./install.sh
    ./install.sh -l
    
### Root
    git clone https://github.com/lighttigerXIV/catppuccinifier.git
    cd catppuccinifier
    chmod +x ./install.sh
    ./install.sh
    
## How to use

### Parameters
Parameter | Abv. | Description |
|-------|-----------|-----|
| **latte** | **-l** | Creates a image with the latte lut |
| **frappe** | **-f** | Creates a image with the frappe lut |
| **macchiato** | **-ma** | Creates a image with the macchiato lut |
| **mocha** | **-mo** | Creates a image with the mocha lut |
| **oled** | **-o** | Creates a image with the oled lut |
| **all** | **-a** | Creates multiple images with all lut |
| **noise-1** | **-n1** | Sets the attenuate level to 1 |
| **noise-2** | **-n2** | Sets the attenuate level to 2 |
| **noise-3** | **-n3** | Sets the attenuate level to 3 |
| **noise-4** | **-n4** | Sets the attenuate level to 4 |
| **version** | **-v** | Shows the current program version |

**Note:** Default noise level is 0

### Examples

```catppuccinifier mocha oled n2 your_image.png```

```catppuccinifier all your_image.png```

```catppuccinifier mocha noise-3 your_image.png```

## Thanks
- [lighttigerXIV](https://github.com/lighttigerXIV)
- [Gingeh](https://github.com/Gingeh)

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
To install catppuccinifier you can use steps presented in the [INSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/INSTALL.md)

## Uninstall
To uninstall catppuccinifier you can use steps presented in the [UNINSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/UNINSTALL.md)

## Other amazing people that worked on the project
- [Isabel](https://github.com/isabelroses)
- [Ozwaldorf](https://github.com/ozwaldorf/lutgen-rs)

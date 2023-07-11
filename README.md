<h1 align="center">Catppuccinifier</h1>

## About
This project gives you a cli and a gui for catppuccinifying your wallpapers. It's available for Linux and Windows.

<div align="center">
<img src="https://github.com/lighttigerXIV/catppuccinifier/assets/35658492/6813d2c0-f45b-4871-b2d3-332655b07e57" width="600">
<img src="https://user-images.githubusercontent.com/35658492/235938647-79fa5eef-a5e4-4f32-ad1b-ab265d7011cb.png" width="600">
</div>


## How to use cli
The cli version works by selecting the flavor, the noise level and the image to be generated.

|Short|Full|Possible Values|Description|
------|----|---------------|-----------|
|-f|--flavor|latte frappe macchiato mocha oled all| Selects the flavor (multiple can be used) [Default: all]|
|-i|--image| * | Selects the image to generate from. [Field Required] |
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
    catppuccinifier -i fuji.jpg

    catppuccinifier -f mocha macchiato -n 3 -i fuji.jpg

## Installation
To install catppuccinifier you can use steps presented in the [INSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/INSTALL.md)

> **note** <br>
> Nix users can use https://isabelroses.cachix.org to access prebuilt binaries

## Uninstall
To uninstall catppuccinifier you can use steps presented in the [UNINSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/UNINSTALL.md)

## Other amazing people that worked on the project
- [Isabel](https://github.com/isabelroses)
- [Ozwaldorf](https://github.com/ozwaldorf/lutgen-rs)

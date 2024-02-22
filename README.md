<h1 align="center">Catppuccinifier</h1>

## About
This project gives you a cli and a gui for catppuccinifying your wallpapers. It's available for Linux and Windows. It uses [lutgen](https://github.com/ozwaldorf/lutgen-rs) under the hood

<div align="center">
<h3>GUI</h3>
<img src="https://github.com/lighttigerXIV/catppuccinifier/assets/35658492/047878ee-306f-45e6-9ad7-c814778bc55a" width="600">
<h3>CLI</h3>
<img src="https://github.com/lighttigerXIV/catppuccinifier/assets/35658492/84e13e34-ecea-4500-b794-6b5122b6bbf2" width="600">
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

## Uninstall
To uninstall catppuccinifier you can use steps presented in the [UNINSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/UNINSTALL.md)

## Other amazing people that worked on the project
- [Isabel](https://github.com/isabelroses)
- [Ozwaldorf](https://github.com/ozwaldorf/lutgen-rs)

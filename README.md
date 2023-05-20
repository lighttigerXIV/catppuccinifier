<h1 align="center">Catppuccinifier</h1>

## About
This project gives you a cli and a gui for catppuccinifying your wallpapers. It's available for Linux and Windows.

<div align="center">
<img src="https://github.com/lighttigerXIV/catppuccinifier/assets/35658492/8b8a96c3-9837-4d19-9375-848831b27536" width="600">
<img src="https://user-images.githubusercontent.com/35658492/235938647-79fa5eef-a5e4-4f32-ad1b-ab265d7011cb.png" width="600">
</div>



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
To install catppuccinifier you can use steps presented in the [INSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/INSTALL.md)

## Uninstall
To uninstall catppuccinifier you can use steps presented in the [UNINSTALL.md](https://github.com/lighttigerXIV/catppuccinifier/blob/master/UNINSTALL.md)

## Other amazing people that worked on the project
- [Isabel](https://github.com/isabelroses)

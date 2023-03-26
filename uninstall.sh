#!/bin/bash

if [ -f /usr/local/bin/catppuccinifier ];then

    sudo rm /usr/local/bin/catppuccinifier

fi

if [ -f /usr/local/bin/catppuccinifier-gui ];then

    sudo rm /usr/local/bin/catppuccinifier-gui

fi

if [ -f "$HOME"/.local/bin/catppuccinifier ];then

    rm "$HOME"/.local/bin/catppuccinifier

fi

if [ -f "$HOME"/.local/bin/catppuccinifier-gui ];then

    rm "$HOME"/.local/bin/catppuccinifier-gui

fi

if [ -d "$HOME"/.local/share/catppuccinifier ];then

    rm -rf "$HOME"/.local/share/catppuccinifier

fi

if [ -f "$HOME"/.local/share/icons/hicolor/512x512/apps/catppuccinifier.png ];then

    rm -rf "$HOME"/.local/share/icons/hicolor/512x512/apps/catppuccinifier.png

fi

if [ -f "$HOME"/.local/share/applications/Catppuccinifier.desktop ];then

    rm -rf "$HOME"/.local/share/applications/Catppuccinifier.desktop

fi
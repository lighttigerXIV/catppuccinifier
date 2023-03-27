#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

#Installs the binaries locally
if [[ "$*" == "--local" ]] || [[ "$*" == "-l" ]]; then

    cp "$SCRIPT_DIR"/bin/catppuccinifier "$HOME"/.local/bin/
    cp "$SCRIPT_DIR"/bin/catppuccinifier-gui "$HOME"/.local/bin/
    sed -i "s|Exec=.*|Exec=$HOME/.local/bin/catppuccinifier-gui|g" desktop/Catppuccinifier.desktop 

#Installs the binaries on /usr/
else

    sudo cp -p -r "$SCRIPT_DIR"/bin/catppuccinifier /usr/local/bin/
    sudo cp -p -r "$SCRIPT_DIR"/bin/catppuccinifier-gui /usr/local/bin/
    sed -i "s|Exec=.*|Exec=/usr/local/bin/catppuccinifier-gui|g" desktop/Catppuccinifier.desktop 

fi

#Creates the catppuccinifier folder with the flavors
if ! [ -d "$HOME"/.local/share/catppuccinifier ]; then

    mkdir "$HOME"/.local/share/catppuccinifier
fi

cp -p -r "$SCRIPT_DIR"/src/flavors/ "$HOME"/.local/share/catppuccinifier/

#Copies the icon to the apps folder
if ! [ -d "$HOME/.local/share/icons/hicolor/512x512/apps/" ];then

    mkdir -p "$HOME/.local/share/icons/hicolor/512x512/apps/"

fi

cp "$SCRIPT_DIR"/images/catppuccinifier.png "$HOME"/.local/share/icons/hicolor/512x512/apps/

#Installs the .desktop file
sed -i "s|Icon=.*|Icon=$HOME/.local/share/icons/hicolor/512x512/apps/catppuccinifier.png|g" desktop/Catppuccinifier.desktop
cp "$SCRIPT_DIR/desktop/Catppuccinifier.desktop" "$HOME/.local/share/applications/"

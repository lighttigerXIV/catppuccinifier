#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

if [[ "$*" == "--local" ]] || [[ "$*" == "-l" ]]; then

    cp "$SCRIPT_DIR"/bin/catppuccinifier "$HOME"/.local/bin/
    cp "$SCRIPT_DIR"/bin/catppuccinifier-gui "$HOME"/.local/bin/

else

    sudo cp -p -r "$SCRIPT_DIR"/bin/catppuccinifier /usr/local/bin/
    sudo cp -p -r "$SCRIPT_DIR"/bin/catppuccinifier-gui /usr/local/bin/

fi

if ! [ -d "$HOME"/.local/share/catppuccinifier ]; then

    mkdir "$HOME"/.local/share/catppuccinifier
fi

cp -p -r "$SCRIPT_DIR"/src/flavors/ "$HOME"/.local/share/catppuccinifier/
cp "$SCRIPT_DIR"/images/catppuccinifier.png "$HOME"/.local/share/icons/hicolor/512x512/apps/
sed -i "s|Icon=.*|Icon=$HOME/.local/share/icons/hicolor/512x512/catppuccinifier.png|g" desktop/Catppuccinifier.desktop 
cp "$SCRIPT_DIR/desktop/Catppuccinifier.desktop" "$HOME/.local/share/applications/"
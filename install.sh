#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

if ! which python3 >/dev/null; then
    
    echo "Please install python3"

elif ! which magick >/dev/null; then
    
    echo "Please install image magick"

else

    if ! [ -d ~/.local/share/catppuccinifier/ ]; then

        sudo mkdir ~/.local/share/catppuccinifier
    fi

    sudo cp -p -r $SCRIPT_DIR/src/flavors/ ~/.local/share/catppuccinifier/

    sudo cp -p $SCRIPT_DIR/bin/catppuccinifier /usr/local/bin/
fi

#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

if ! which python3 >/dev/null; then
    
    echo "Please install python3"

elif ! which magick >/dev/null; then
    
    echo "Please install image magick"

else

    if ! [ -d /opt/catppuccinifier/ ]; then

        sudo mkdir /opt/catppuccinifier
    fi

    sudo cp -p -r $SCRIPT_DIR/src/flavors/ /opt/catppuccinifier/

    sudo cp -p $SCRIPT_DIR/bin/catppuccinifier /usr/local/bin/
fi
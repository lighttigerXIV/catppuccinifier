SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"

if [[ "$*" == "--local" ]] || [[ "$*" == "-l" ]]; then

    cp $SCRIPT_DIR/bin/catppuccinifier $HOME/.local/bin/

else

    sudo cp -p -r $SCRIPT_DIR/bin/catppuccinifier /usr/local/bin/

fi

if ! [ -d $HOME/.local/share/catppuccinifier ]; then

    mkdir $HOME/.local/share/catppuccinifier
fi

cp -p -r $SCRIPT_DIR/src/flavors/ $HOME/.local/share/catppuccinifier/

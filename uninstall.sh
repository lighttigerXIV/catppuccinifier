#!/bin/bash

if [ -f /usr/local/bin/catppuccinifier ];then

    sudo rm /usr/local/bin/catppuccinifier

fi

if [ -f $HOME/.local/bin/catppuccinifier ];then

    rm $HOME/.local/bin/catppuccinifier

fi

if [ -d $HOME/.local/share/catppuccinifier ];then

    rm -rf $HOME/.local/share/catppuccinifier

fi
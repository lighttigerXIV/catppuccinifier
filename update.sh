#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"


bash $SCRIPT_DIR/uninstall.sh

if [[ "$*" == "--local" ]] || [[ "$*" == "-l" ]]; then
    bash $SCRIPT_DIR/install.sh -l
else
    bash $SCRIPT_DIR/install.sh
fi
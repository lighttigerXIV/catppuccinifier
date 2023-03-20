SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd)"




bash $SCRIPT_DIR/uninstall.sh
bash $SCRIPT_DIR/install.sh
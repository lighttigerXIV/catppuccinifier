CURRENT_DIR=$(pwd)

echo "OLED"
convert HALD:8 -duplicate 512 -attenuate $1 +noise Gaussian -quantize LAB +dither -remap $CURRENT_DIR/palette/catppuccin-oled.png -evaluate-sequence Mean $CURRENT_DIR/oled.png
echo "MOCHA"
convert HALD:8 -duplicate 512 -attenuate $1 +noise Gaussian -quantize LAB +dither -remap $CURRENT_DIR/palette/catppuccin-mocha.png -evaluate-sequence Mean $CURRENT_DIR/mocha.png
echo "MACCHIATO"
convert HALD:8 -duplicate 512 -attenuate $1 +noise Gaussian -quantize LAB +dither -remap $CURRENT_DIR/palette/catppuccin-macchiato.png -evaluate-sequence Mean $CURRENT_DIR/macchiato.png
echo "FRAPPE"
convert HALD:8 -duplicate 512 -attenuate $1 +noise Gaussian -quantize LAB +dither -remap $CURRENT_DIR/palette/catppuccin-frappe.png -evaluate-sequence Mean $CURRENT_DIR/frappe.png
echo "LATTE"
convert HALD:8 -duplicate 512 -attenuate $1 +noise Gaussian -quantize LAB +dither -remap $CURRENT_DIR/palette/catppuccin-latte.png -evaluate-sequence Mean $CURRENT_DIR/latte.png

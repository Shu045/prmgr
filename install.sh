#!/bin/bash

set -e

VERSION="v1.0.0"
URL="https://github.com/Shu045/prmgr/releases/download/${VERSION}/prmgr"

INSTALL_DIR="$HOME/.prmgr/bin"
BINARY="$INSTALL_DIR/prmgr"

mkdir -p "$INSTALL_DIR"

echo "Downloading prmgr..."
curl -fsSL "$URL" -o "$BINARY"

chmod +x "$BINARY"

echo "Installed to $BINARY"

# Detect the user's login shell
LOGIN_SHELL=$(basename "$SHELL")

case "$LOGIN_SHELL" in
    zsh)
        SHELL_RC="$HOME/.zshrc"
        ;;
    bash)
        SHELL_RC="$HOME/.bashrc"
        ;;
    fish)
        SHELL_RC="$HOME/.config/fish/config.fish"
        ;;
    *)
        SHELL_RC="$HOME/.profile"
        ;;
esac

PATH_LINE='export PATH="$HOME/.prmgr/bin:$PATH"'

# Add PATH only if it doesn't already exist
if [ -f "$SHELL_RC" ]; then
    if ! grep -Fxq "$PATH_LINE" "$SHELL_RC"; then
        echo "" >> "$SHELL_RC"
        echo "$PATH_LINE" >> "$SHELL_RC"
        echo "Added ~/.prmgr/bin to PATH in $SHELL_RC"
    fi
else
    echo "$PATH_LINE" > "$SHELL_RC"
    echo "Created $SHELL_RC and added ~/.prmgr/bin to PATH"
fi

echo
echo "Installation complete!"
echo
echo "Run the following command or restart your terminal:"
echo
echo "source $SHELL_RC"
echo
echo "run: prmgr"

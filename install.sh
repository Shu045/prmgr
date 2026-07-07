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

# Detect shell configuration file
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
    SHELL_NAME="zsh"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
    SHELL_NAME="bash"
else
    SHELL_RC="$HOME/.profile"
    SHELL_NAME="your shell"
fi

# Add to PATH if not already present
if ! grep -q 'export PATH="$HOME/.prmgr/bin:$PATH"' "$SHELL_RC" 2>/dev/null; then
    echo 'export PATH="$HOME/.prmgr/bin:$PATH"' >> "$SHELL_RC"
    echo "Added ~/.prmgr/bin to PATH in $SHELL_RC"
fi

echo
echo "Installation complete!"
echo
echo "Restart your terminal or run:"
echo "source $SHELL_RC"
echo
echo "Then start prmgr with:"
echo "prmgr"

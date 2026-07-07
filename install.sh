#!/bin/bash

set -e

VERSION="v1.0.0"
URL="https://github.com/Shu045/prmgr/releases/download/${VERSION}/prmgr"

INSTALL_DIR="$HOME/.prmgr/bin"

mkdir -p "$INSTALL_DIR"

echo "Downloading..."
curl -L "$URL" -o "$INSTALL_DIR/prmgr"

chmod +x "$INSTALL_DIR/prmgr"

echo "Installed to $INSTALL_DIR"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo 'export PATH="$HOME/.prmgr/bin:$PATH"' >> ~/.bashrc
    echo "Added ~/.prmgr/bin to PATH"
    echo "Restart your terminal or run:"
    echo "source ~/.bashrc"
fi

echo "Done! Run:"
echo "prmgr"

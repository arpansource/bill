#!/bin/bash

set -e

echo "📦 Installing bill..."

# Detect OS
OS="$(uname -s)"

case "$OS" in
  Darwin)
    FILE="bill-macos"
    ;;
  Linux)
    FILE="bill-linux"
    ;;
  *)
    echo "❌ Unsupported OS: $OS"
    exit 1
    ;;
esac

# Temp directory
TMP_DIR=$(mktemp -d)
cd "$TMP_DIR"

# Download binary
echo "⬇️ Downloading latest release..."
curl -L -o bill "https://github.com/arpansource/bill/releases/download/v0.1.0/$FILE"

# Make executable
chmod +x bill

# Decide install directory
INSTALL_DIR="/usr/local/bin"

if [ ! -w "$INSTALL_DIR" ]; then
  echo "🔐 Need sudo permissions to install to $INSTALL_DIR"
  sudo mv bill "$INSTALL_DIR/bill"
else
  mv bill "$INSTALL_DIR/bill"
fi

echo "✅ bill installed successfully!"
echo ""
echo "Run:"
echo "  bill init"

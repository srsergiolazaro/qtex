#!/bin/bash

# Abort on error
set -e

# --- Configuration ---
INSTALL_DIR="$HOME/.qtex"
BIN_DIR="$INSTALL_DIR/bin"
BINARY_NAME="qtex"
REPO="srsergiolazaro/qtex"

# --- Colors ---
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
GREEN='\033[0;32m'
BOLD='\033[1m'
RESET='\033[0m'

echo -e "${MAGENTA}${BOLD}🌀 qtex Installer${RESET}\n"

# 1. Detect OS and Architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
    darwin)
        if [ "$ARCH" = "arm64" ]; then
            ASSET_NAME="qtex-darwin-arm64"
        else
            ASSET_NAME="qtex-darwin-x64"
        fi
        ;;
    linux)
        ASSET_NAME="qtex-linux-x64"
        ;;
    *)
        echo -e "❌ Unsupported OS: $OS. Please install manually or use Windows installer."
        exit 1
        ;;
esac

# 2. Preparation & Clean Up
# Resolve conflict with old hybrid installation
if [ -d "$INSTALL_DIR/runtime" ]; then
    echo -e "${MAGENTA}⚠️  Detected old hybrid installation. Cleaning up...${RESET}"
    rm -rf "$INSTALL_DIR"
fi

mkdir -p "$BIN_DIR"

# 3. Download standalone binary from GitHub
echo -e "${BLUE}🚚 Downloading $ASSET_NAME (Standalone binary)...${RESET}"
URL="https://github.com/$REPO/releases/latest/download/$ASSET_NAME"

if ! curl -sSL -o "$BIN_DIR/$BINARY_NAME" "$URL"; then
    echo -e "❌ Download failed from GitHub. Please check your connection."
    exit 1
fi

chmod +x "$BIN_DIR/$BINARY_NAME"

# 4. Add to PATH
SHELL_CONFIG=""
case $SHELL in
    */zsh) SHELL_CONFIG="$HOME/.zshrc" ;;
    */bash) SHELL_CONFIG="$HOME/.bashrc" ;;
    *) 
        if [ -f "$HOME/.zshrc" ]; then SHELL_CONFIG="$HOME/.zshrc";
        elif [ -f "$HOME/.bashrc" ]; then SHELL_CONFIG="$HOME/.bashrc";
        fi
        ;;
esac

if [ -n "$SHELL_CONFIG" ]; then
    if ! grep -q "$BIN_DIR" "$SHELL_CONFIG"; then
        echo -e "${BLUE}⚙️  Adding $BIN_DIR to PATH in $SHELL_CONFIG...${RESET}"
        echo "" >> "$SHELL_CONFIG"
        echo "# qtex binary" >> "$SHELL_CONFIG"
        echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_CONFIG"
    else
        echo -e "✅ $BIN_DIR is already in your PATH."
    fi
fi

echo -e "\n${GREEN}${BOLD}✨ qtex installed successfully!${RESET}"
echo -e "Please run ${BOLD}source $SHELL_CONFIG${RESET} to start using 'qtex'."
echo -e "Usage example: ${BLUE}qtex ./example --watch${RESET}"

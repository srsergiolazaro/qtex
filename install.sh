#!/bin/bash

# Abort on error
set -e

# --- Configuration ---
INSTALL_DIR="$HOME/.qtex"
RUNTIME_DIR="$INSTALL_DIR/runtime"
BIN_DIR="$INSTALL_DIR/bin"
QTEX_JS="$INSTALL_DIR/qtex.js"
SHIM_PATH="$BIN_DIR/qtex"
REPO="srsergiolazaro/qtex"

# --- Colors ---
BLUE='\033[0;34m'
MAGENTA='\033[0;35m'
GREEN='\033[0;32m'
BOLD='\033[1m'
RESET='\033[0m'

echo -e "${MAGENTA}${BOLD}🌀 qtex Installer (Hybrid Architecture)${RESET}\n"

# 1. Environment Verification & Deep Clean
if [ -d "$INSTALL_DIR" ]; then
    IS_LEGACY=false
    if [ -f "$BIN_DIR/qtex.exe" ] || [ -f "$BIN_DIR/qtex-bin" ]; then IS_LEGACY=true; fi
    
    IS_BROKEN=false
    if [ -f "$SHIM_PATH" ]; then
        if ! "$SHIM_PATH" --version >/dev/null 2>&1; then IS_BROKEN=true; fi
    else
        # If folder exists but no shim and no legacy bin, it's broken
        if [ "$IS_LEGACY" = "false" ]; then IS_BROKEN=true; fi
    fi

    if [ "$IS_LEGACY" = "true" ] || [ "$IS_BROKEN" = "true" ]; then
        echo -e "${MAGENTA}⚠️  Very old or broken version detected. Performing deep clean...${RESET}"
        rm -rf "$INSTALL_DIR"
    fi
fi

mkdir -p "$RUNTIME_DIR"
mkdir -p "$BIN_DIR"

# 2. Install Bun (The "Motor")
BUN_BIN="$RUNTIME_DIR/bun"

# Check if existing Bun is broken
if [ -f "$BUN_BIN" ]; then
    if ! "$BUN_BIN" --version >/dev/null 2>&1; then
        echo -e "${MAGENTA}⚠️  Existing Bun runtime is broken or incompatible. Reinstalling...${RESET}"
        rm -f "$BUN_BIN"
    fi
fi

if [ ! -f "$BUN_BIN" ]; then
    echo -e "${BLUE}⚙️  Installing Bun Runtime (First time only)...${RESET}"
    
    # Detect OS/Arch
    OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
    ARCH="$(uname -m)"
    
    if [ "$OS" = "darwin" ]; then
        if [ "$ARCH" = "arm64" ]; then TARGET="bun-darwin-aarch64"; else TARGET="bun-darwin-x64"; fi
    else
        TARGET="bun-linux-x64"
    fi

    # Disable pushd/popd output
    pushd "$RUNTIME_DIR" > /dev/null
    
    # Download and extract Bun
    curl -fsSL "https://github.com/oven-sh/bun/releases/latest/download/$TARGET.zip" -o bun.zip
    unzip -q bun.zip
    mv "$TARGET/bun" ./bun
    chmod +x ./bun
    
    # Cleanup
    rm bun.zip
    rm -rf "$TARGET"
    
    popd > /dev/null
fi

# 3. Download qtex bundle (The "Cartridge")
echo -e "${BLUE}📦 Downloading latest qtex bundle...${RESET}"
if ! curl -fsSL "https://github.com/$REPO/releases/latest/download/qtex.js" -o "$QTEX_JS"; then
    if [ -d "$INSTALL_DIR" ]; then
        echo -e "${MAGENTA}⚠️  Update failed. Wiping root directory to resolve conflicts...${RESET}"
        rm -rf "$INSTALL_DIR"
        echo -e "${BLUE}✨ Root cleaned. Please re-run the install command to complete fresh installation.${RESET}"
        echo -e "   curl -fsSL https://srsergiolazaro.github.io/qtex/install.sh | bash"
        exit 1
    fi
    echo -e "${MAGENTA}❌ Failed to download qtex.js from latest release.${RESET}"
    exit 1
fi

# 4. Create Shim (The "Key")
echo -e "${BLUE}🔌 Creating entry point...${RESET}"
cat <<EOF > "$SHIM_PATH"
#!/bin/bash
exec "$BUN_BIN" run "$QTEX_JS" "\$@"
EOF
chmod +x "$SHIM_PATH"

# 5. Add to PATH
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

echo -e "\n${GREEN}${BOLD}✨ qtex installed! Update size reduced by 1000x.${RESET}"
echo -e "Please run ${BOLD}source $SHELL_CONFIG${RESET} to start using qtex."

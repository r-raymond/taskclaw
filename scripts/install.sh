#!/bin/bash
set -e

# Claw Installation Script
# Usage: curl -sSL https://raw.githubusercontent.com/r-raymond/taskclaw/main/scripts/install.sh | bash

REPO="r-raymond/taskclaw"
BINARY_NAME="claw"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Installing $BINARY_NAME...${NC}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        case "$ARCH" in
            x86_64) PLATFORM="linux-x86_64" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        ARCHIVE_EXT="tar.gz"
        ;;
    Darwin*)
        case "$ARCH" in
            x86_64) PLATFORM="macos-x86_64" ;;
            arm64) PLATFORM="macos-arm64" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        ARCHIVE_EXT="tar.gz"
        ;;
    *)
        echo -e "${RED}Unsupported operating system: $OS${NC}"
        exit 1
        ;;
esac

# Get the latest release version
echo -e "${YELLOW}Fetching latest release...${NC}"
LATEST_VERSION=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

if [ -z "$LATEST_VERSION" ]; then
    echo -e "${RED}Failed to get latest version${NC}"
    exit 1
fi

echo -e "${GREEN}Latest version: $LATEST_VERSION${NC}"

# Download URL
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_VERSION/$BINARY_NAME-$PLATFORM.$ARCHIVE_EXT"

# Create temp directory
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

echo -e "${YELLOW}Downloading $BINARY_NAME $LATEST_VERSION for $PLATFORM...${NC}"

# Download and extract
cd "$TMP_DIR"
if ! curl -L -o "archive.$ARCHIVE_EXT" "$DOWNLOAD_URL"; then
    echo -e "${RED}Failed to download $BINARY_NAME${NC}"
    exit 1
fi

echo -e "${YELLOW}Extracting archive...${NC}"
tar -xzf "archive.$ARCHIVE_EXT"

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Install binary
echo -e "${YELLOW}Installing to $INSTALL_DIR...${NC}"
cp "$BINARY_NAME" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Install man page if available
if [ -f "man/$BINARY_NAME.1" ]; then
    MAN_DIR="$HOME/.local/share/man/man1"
    mkdir -p "$MAN_DIR"
    cp "man/$BINARY_NAME.1" "$MAN_DIR/"
    echo -e "${GREEN}Installed man page to $MAN_DIR${NC}"
fi

# Install shell completions if available
if [ -d "completions" ]; then
    # Bash completion
    if [ -f "completions/$BINARY_NAME.bash" ]; then
        BASH_COMPLETION_DIR="$HOME/.local/share/bash-completion/completions"
        mkdir -p "$BASH_COMPLETION_DIR"
        cp "completions/$BINARY_NAME.bash" "$BASH_COMPLETION_DIR/$BINARY_NAME"
        echo -e "${GREEN}Installed bash completion${NC}"
    fi
    
    # Zsh completion
    if [ -f "completions/_$BINARY_NAME" ]; then
        ZSH_COMPLETION_DIR="$HOME/.local/share/zsh/site-functions"
        mkdir -p "$ZSH_COMPLETION_DIR"
        cp "completions/_$BINARY_NAME" "$ZSH_COMPLETION_DIR/"
        echo -e "${GREEN}Installed zsh completion${NC}"
    fi
    
    # Fish completion
    if [ -f "completions/$BINARY_NAME.fish" ]; then
        FISH_COMPLETION_DIR="$HOME/.config/fish/completions"
        mkdir -p "$FISH_COMPLETION_DIR"
        cp "completions/$BINARY_NAME.fish" "$FISH_COMPLETION_DIR/"
        echo -e "${GREEN}Installed fish completion${NC}"
    fi
fi

echo -e "${GREEN}$BINARY_NAME installed successfully!${NC}"

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Warning: $INSTALL_DIR is not in your PATH${NC}"
    echo -e "${YELLOW}Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):${NC}"
    echo -e "${BLUE}export PATH=\"$INSTALL_DIR:\$PATH\"${NC}"
fi

echo -e "${GREEN}Run '$BINARY_NAME --help' to get started!${NC}"

#!/bin/bash
set -e

# Download standalone static binary script
# Usage: curl -sSL https://raw.githubusercontent.com/r-raymond/taskclaw/main/scripts/download-binary.sh | bash

REPO="r-raymond/taskclaw"
BINARY_NAME="claw"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Downloading standalone $BINARY_NAME binary...${NC}"

# Detect OS and architecture
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        case "$ARCH" in
            x86_64) PLATFORM="linux-x86_64-static" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        ;;
    Darwin*)
        case "$ARCH" in
            x86_64) PLATFORM="macos-x86_64-static" ;;
            arm64) PLATFORM="macos-arm64-static" ;;
            *) echo -e "${RED}Unsupported architecture: $ARCH${NC}"; exit 1 ;;
        esac
        ;;
    *)
        echo -e "${RED}Unsupported operating system: $OS${NC}"
        echo -e "${YELLOW}For Windows, download: claw-windows-x86_64-static.exe${NC}"
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

# Download URL for standalone binary
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_VERSION/$BINARY_NAME-$PLATFORM"

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

echo -e "${YELLOW}Downloading standalone binary...${NC}"

# Download the binary directly
if ! curl -L -o "$INSTALL_DIR/$BINARY_NAME" "$DOWNLOAD_URL"; then
    echo -e "${RED}Failed to download $BINARY_NAME${NC}"
    exit 1
fi

# Make it executable
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo -e "${GREEN}$BINARY_NAME standalone binary installed successfully!${NC}"

# Verify the binary
echo -e "${YELLOW}Verifying installation...${NC}"
if "$INSTALL_DIR/$BINARY_NAME" --version > /dev/null 2>&1; then
    echo -e "${GREEN}✓ Binary is working correctly${NC}"
else
    echo -e "${RED}✗ Binary verification failed${NC}"
    exit 1
fi

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "${YELLOW}Warning: $INSTALL_DIR is not in your PATH${NC}"
    echo -e "${YELLOW}Add the following line to your shell profile (~/.bashrc, ~/.zshrc, etc.):${NC}"
    echo -e "${BLUE}export PATH=\"$INSTALL_DIR:\$PATH\"${NC}"
fi

echo -e "${GREEN}Run '$BINARY_NAME --help' to get started!${NC}"
echo -e "${BLUE}This is a standalone static binary with no dependencies.${NC}"

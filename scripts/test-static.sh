#!/bin/bash
set -e

echo "Testing static compilation..."

# Test if we can build a static binary
echo "Building static binary for current platform..."

# Determine current platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
    Linux*)
        case "$ARCH" in
            x86_64) TARGET="x86_64-unknown-linux-musl" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    Darwin*)
        case "$ARCH" in
            x86_64) TARGET="x86_64-apple-darwin" ;;
            arm64) TARGET="aarch64-apple-darwin" ;;
            *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
        esac
        ;;
    *)
        echo "Unsupported operating system: $OS"
        exit 1
        ;;
esac

echo "Building for target: $TARGET"

# Install target if needed
rustup target add "$TARGET" || echo "Target already installed"

# Build static binary
echo "Building with static profile..."
cargo build --profile release-static --target "$TARGET"

BINARY_PATH="target/$TARGET/release-static/claw"

if [ -f "$BINARY_PATH" ]; then
    echo "✅ Static binary built successfully"
    
    echo "Binary info:"
    file "$BINARY_PATH"
    
    echo "Binary size:"
    ls -lah "$BINARY_PATH"
    
    if [ "$OS" = "Linux" ]; then
        echo "Checking dynamic dependencies (should be minimal or none):"
        ldd "$BINARY_PATH" || echo "✅ No dynamic dependencies (fully static)"
    elif [ "$OS" = "Darwin" ]; then
        echo "Checking dynamic dependencies:"
        otool -L "$BINARY_PATH"
    fi
    
    echo "Testing binary functionality:"
    "$BINARY_PATH" --version
    echo "✅ Binary works correctly"
    
else
    echo "❌ Static binary not found"
    exit 1
fi

echo "Static compilation test completed successfully!"
#!/bin/bash
set -e

# Script to verify static binaries work on clean systems
# This simulates what happens when users download standalone binaries

echo "🔍 Verifying static binary compatibility..."

BINARY_PATH="${1:-target/release-static/claw}"

if [ ! -f "$BINARY_PATH" ]; then
    echo "❌ Binary not found at: $BINARY_PATH"
    echo "Usage: $0 [binary_path]"
    exit 1
fi

echo "📁 Testing binary: $BINARY_PATH"

# Check if binary exists and is executable
if [ ! -x "$BINARY_PATH" ]; then
    echo "❌ Binary is not executable"
    exit 1
fi

echo "✅ Binary is executable"

# Test basic functionality
echo "🧪 Testing basic functionality..."

# Test version command
if "$BINARY_PATH" --version > /dev/null 2>&1; then
    VERSION=$("$BINARY_PATH" --version)
    echo "✅ Version command works: $VERSION"
else
    echo "❌ Version command failed"
    exit 1
fi

# Test help command
if "$BINARY_PATH" --help > /dev/null 2>&1; then
    echo "✅ Help command works"
else
    echo "❌ Help command failed"
    exit 1
fi

# Test in a temporary directory (simulating clean environment)
TEMP_DIR=$(mktemp -d)
trap 'rm -rf "$TEMP_DIR"' EXIT

cd "$TEMP_DIR"
echo "📂 Testing in clean directory: $TEMP_DIR"

# Copy binary to temp location
cp "$BINARY_PATH" "./claw-test"

# Test basic task operations
echo "🧪 Testing task operations..."

if ./claw-test add "Test task" > /dev/null 2>&1; then
    echo "✅ Add task works"
else
    echo "❌ Add task failed"
    exit 1
fi

if ./claw-test list | grep -q "Test task"; then
    echo "✅ List tasks works"
else
    echo "❌ List tasks failed"
    exit 1
fi

if ./claw-test complete 0 > /dev/null 2>&1; then
    echo "✅ Complete task works"
else
    echo "❌ Complete task failed"
    exit 1
fi

if ./claw-test remove 0 > /dev/null 2>&1; then
    echo "✅ Remove task works"
else
    echo "❌ Remove task failed"
    exit 1
fi

# Test completions generation
echo "🧪 Testing completion generation..."
if ./claw-test completions bash > /dev/null 2>&1; then
    echo "✅ Bash completions work"
else
    echo "❌ Bash completions failed"
    exit 1
fi

echo ""
echo "🎉 All tests passed! Binary is working correctly."
echo "📊 Binary info:"
echo "   Size: $(ls -lah ./claw-test | awk '{print $5}')"
echo "   Type: $(file ./claw-test | cut -d: -f2-)"

# Platform-specific dependency checks
OS="$(uname -s)"
if [ "$OS" = "Linux" ]; then
    echo "🔗 Linux dependency check:"
    if command -v ldd >/dev/null 2>&1; then
        ldd ./claw-test 2>/dev/null || echo "   ✅ No dynamic dependencies (fully static)"
    fi
elif [ "$OS" = "Darwin" ]; then
    echo "🔗 macOS dependency check:"
    if command -v otool >/dev/null 2>&1; then
        echo "   Dependencies:"
        otool -L ./claw-test | grep -v "claw-test:" | sed 's/^/     /'
    fi
fi

echo ""
echo "✅ Static binary verification completed successfully!"
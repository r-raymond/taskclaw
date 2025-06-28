#!/bin/bash
set -e

echo "Testing package building..."

# Build the release binary first
echo "Building release binary..."
cargo build --release

echo "Testing man page and completion generation..."
ls -la target/release/build/*/out/man/ || echo "Man pages not found"
ls -la target/release/build/*/out/completions/ || echo "Completions not found"

# Test cargo-deb if available
if command -v cargo-deb &> /dev/null; then
    echo "Testing deb package creation..."
    cargo deb --no-build
    echo "✅ Deb package created successfully"
    ls -la target/debian/
else
    echo "⚠️  cargo-deb not installed, skipping deb test"
    echo "Install with: cargo install cargo-deb"
fi

# Test cargo-generate-rpm if available  
if command -v cargo-generate-rpm &> /dev/null; then
    echo "Testing RPM package creation..."
    cargo generate-rpm
    echo "✅ RPM package created successfully"
    ls -la target/generate-rpm/
else
    echo "⚠️  cargo-generate-rpm not installed, skipping RPM test"
    echo "Install with: cargo install cargo-generate-rpm"
fi

echo "Package testing complete!"
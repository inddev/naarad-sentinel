#!/bin/bash
# build-all.sh - Cross-compile Naarad Sentinel for all platforms

set -e

echo "🚀 Building Naarad Sentinel for all platforms..."
echo

# Add targets
rustup target add x86_64-unknown-linux-gnu
rustup target add aarch64-unknown-linux-gnu  
rustup target add armv7-unknown-linux-gnueabihf
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Clean previous builds
cargo clean
mkdir -p dist

echo "📦 Building for Linux x64..."
cargo build --release --target x86_64-unknown-linux-gnu
cp target/x86_64-unknown-linux-gnu/release/naarad-sentinel dist/naarad-sentinel-linux-x64

echo "📦 Building for Linux ARM64..."
cargo build --release --target aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/naarad-sentinel dist/naarad-sentinel-linux-arm64

echo "📦 Building for Raspberry Pi (armv7)..."
cargo build --release --target armv7-unknown-linux-gnueabihf
cp target/armv7-unknown-linux-gnueabihf/release/naarad-sentinel dist/naarad-sentinel-raspberry-pi

echo "📦 Building for Windows x64..."
cargo build --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/naarad-sentinel.exe dist/naarad-sentinel-windows-x64.exe

echo "📦 Building for macOS Intel..."
cargo build --release --target x86_64-apple-darwin
cp target/x86_64-apple-darwin/release/naarad-sentinel dist/naarad-sentinel-macos-intel

echo "📦 Building for macOS Apple Silicon..."
cargo build --release --target aarch64-apple-darwin
cp target/aarch64-apple-darwin/release/naarad-sentinel dist/naarad-sentinel-macos-arm

echo
echo "✅ All builds complete! Files in ./dist/"
echo
echo "📋 Built binaries:"
ls -la dist/

echo
echo "🚀 Ready for release!"

#!/bin/bash
# install-pi.sh - One-command Raspberry Pi installation

set -e

echo "🍓 Naarad Sentinel - Raspberry Pi Installer"
echo "==========================================="
echo

# Detect architecture
ARCH=$(uname -m)
if [[ "$ARCH" == "armv7l" ]]; then
    BINARY_NAME="naarad-sentinel-raspberry-pi"
elif [[ "$ARCH" == "aarch64" ]]; then
    BINARY_NAME="naarad-sentinel-linux-arm64"  
else
    echo "❌ Unsupported architecture: $ARCH"
    echo "   Supported: armv7l (Pi 3/4), aarch64 (Pi 4 64-bit)"
    exit 1
fi

echo "📡 Detected architecture: $ARCH"
echo "📦 Binary: $BINARY_NAME"
echo

# Download latest release
echo "⬇️  Downloading latest Sentinel..."
DOWNLOAD_URL="https://github.com/inddev/naarad-sentinel/releases/latest/download/$BINARY_NAME"

curl -L -o naarad-sentinel "$DOWNLOAD_URL"
chmod +x naarad-sentinel

echo "✅ Downloaded successfully!"
echo

# Check if API key provided
if [[ -n "$1" ]]; then
    echo "🔧 Running setup with provided API key..."
    ./naarad-sentinel --setup --api-key="$1"
    echo
    echo "🚀 Setup complete! Starting monitoring..."
    echo "   To run manually: ./naarad-sentinel"
    echo "   To run as service: sudo ./install-service.sh"
else
    echo "📋 Installation complete!"
    echo
    echo "📖 Next steps:"
    echo "   1. Get your API key from https://app.naarad.dev"
    echo "   2. Run setup: ./naarad-sentinel --setup --api-key=YOUR_KEY"
    echo "   3. Start monitoring: ./naarad-sentinel"
fi

echo
echo "🎉 Naarad Sentinel is ready!"

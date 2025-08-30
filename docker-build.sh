#!/bin/bash
# docker-build.sh - Build Naarad Sentinel using Docker for cross-compilation

set -e

echo "🐳 Building Naarad Sentinel with Docker..."
echo

# Create a Dockerfile for building
cat > Dockerfile.build <<EOF
FROM rust:1.75

# Install cross-compilation tools
RUN apt-get update && apt-get install -y \\
    gcc-aarch64-linux-gnu \\
    gcc-arm-linux-gnueabihf \\
    gcc-x86-64-linux-gnu \\
    mingw-w64

# Add Rust targets
RUN rustup target add x86_64-unknown-linux-gnu \\
    && rustup target add aarch64-unknown-linux-gnu \\
    && rustup target add armv7-unknown-linux-gnueabihf \\
    && rustup target add x86_64-pc-windows-gnu

WORKDIR /workspace

# Copy source
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build for all targets
RUN mkdir -p dist

# Linux x64
RUN cargo build --release --target x86_64-unknown-linux-gnu \\
    && cp target/x86_64-unknown-linux-gnu/release/naarad-sentinel dist/naarad-sentinel-linux-x64

# Linux ARM64
RUN CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc \\
    cargo build --release --target aarch64-unknown-linux-gnu \\
    && cp target/aarch64-unknown-linux-gnu/release/naarad-sentinel dist/naarad-sentinel-linux-arm64

# Raspberry Pi (armv7)
RUN CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=arm-linux-gnueabihf-gcc \\
    cargo build --release --target armv7-unknown-linux-gnueabihf \\
    && cp target/armv7-unknown-linux-gnueabihf/release/naarad-sentinel dist/naarad-sentinel-raspberry-pi

# Windows x64
RUN CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER=x86_64-w64-mingw32-gcc \\
    cargo build --release --target x86_64-pc-windows-gnu \\
    && cp target/x86_64-pc-windows-gnu/release/naarad-sentinel.exe dist/naarad-sentinel-windows-x64.exe

# List results
RUN ls -la dist/
EOF

echo "🔨 Building cross-compiled binaries..."
docker build -f Dockerfile.build -t naarad-sentinel-builder .

echo "📦 Extracting binaries..."
docker create --name temp-container naarad-sentinel-builder
docker cp temp-container:/workspace/dist ./dist
docker rm temp-container

# Cleanup
rm -f Dockerfile.build

echo
echo "✅ Cross-compilation complete!"
echo "📁 Binaries available in ./dist/"
ls -la dist/

echo
echo "🎉 Ready for distribution!"

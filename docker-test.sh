#!/bin/bash
# docker-test.sh - Test compilation with Docker

set -e

echo "🧪 Testing Naarad Sentinel compilation in Docker..."

cd /Users/ktn/code/naarad-sentinel

# Quick test build first
echo "🔍 Quick compilation check..."
docker run --rm -v "$(pwd)":/workspace -w /workspace rust:1.75 cargo check

echo "✅ Compilation check passed!"

# Now test a single platform build
echo "🔨 Testing Linux x64 build..."
docker run --rm -v "$(pwd)":/workspace -w /workspace rust:1.75 \
  bash -c "rustup target add x86_64-unknown-linux-gnu && cargo build --release --target x86_64-unknown-linux-gnu"

echo "✅ Docker build test successful!"

# Show the built binary
echo "📁 Built binary:"
ls -la target/x86_64-unknown-linux-gnu/release/naarad-sentinel

echo "🎉 Ready for full cross-compilation!"

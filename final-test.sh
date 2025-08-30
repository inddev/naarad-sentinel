#!/bin/bash
# final-test.sh - Final pre-commit test

set -e

echo "🧪 Final Pre-Commit Test"
echo "======================"
echo

cd /Users/ktn/code/naarad-sentinel

echo "🔍 1. Checking syntax..."
cargo check

echo "✅ Syntax check passed!"
echo

echo "🔨 2. Building release binary..."
cargo build --release

echo "✅ Build successful!"
echo

echo "🧪 3. Testing basic functionality..."
./target/release/naarad-sentinel --help

echo
echo "📁 4. Checking binary size..."
ls -lh target/release/naarad-sentinel

echo
echo "✅ All tests passed! Ready to commit."
echo
echo "📋 Next steps:"
echo "   git add ."
echo "   git commit -m 'feat: add cross-compilation and release automation'"
echo "   git push origin main"
echo "   git tag v0.1.0"
echo "   git push origin v0.1.0"

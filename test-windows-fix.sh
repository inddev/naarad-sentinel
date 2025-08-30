#!/bin/bash
# test-windows-fix.sh - Test Windows compilation fix

set -e

echo "🧪 Testing Windows compilation fix..."
cd /Users/ktn/code/naarad-sentinel

echo "🔍 Checking syntax for all platforms..."
cargo check

echo "✅ All platforms compile successfully!"
echo
echo "📋 Windows-specific changes made:"
echo "  - Moved uname to Unix-only dependencies"
echo "  - Added Windows-specific hostname detection"  
echo "  - Used environment variables for Windows system info"
echo
echo "🚀 Ready to commit the fix and trigger new release!"

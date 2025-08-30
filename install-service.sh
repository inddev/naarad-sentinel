#!/bin/bash  
# install-service.sh - Install Sentinel as systemd service

set -e

if [[ $EUID -ne 0 ]]; then
   echo "❌ This script must be run as root (use sudo)" 
   exit 1
fi

USER_HOME=$(eval echo ~$SUDO_USER)
BINARY_PATH="/usr/local/bin/naarad-sentinel"
CONFIG_PATH="$USER_HOME/.naarad-sentinel/config.json"

echo "🔧 Installing Naarad Sentinel as systemd service..."

# Copy binary to system location
if [[ -f "./naarad-sentinel" ]]; then
    cp ./naarad-sentinel "$BINARY_PATH"
    chmod +x "$BINARY_PATH"
    echo "✅ Binary installed to $BINARY_PATH"
else
    echo "❌ naarad-sentinel binary not found in current directory"
    exit 1
fi

# Check if config exists
if [[ ! -f "$CONFIG_PATH" ]]; then
    echo "❌ Configuration not found at $CONFIG_PATH"
    echo "💡 Run setup first: ./naarad-sentinel --setup --api-key=YOUR_KEY"
    exit 1
fi

# Create systemd service
cat > /etc/systemd/system/naarad-sentinel.service << EOF
[Unit]
Description=Naarad Sentinel Device Monitor
After=network.target

[Service] 
Type=simple
User=$SUDO_USER
ExecStart=$BINARY_PATH
Restart=always
RestartSec=10
Environment=HOME=$USER_HOME

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
systemctl daemon-reload
systemctl enable naarad-sentinel
systemctl start naarad-sentinel

echo "✅ Systemd service installed and started"
echo
echo "📋 Service commands:"
echo "   Status: sudo systemctl status naarad-sentinel"
echo "   Stop:   sudo systemctl stop naarad-sentinel"
echo "   Start:  sudo systemctl start naarad-sentinel"  
echo "   Logs:   sudo journalctl -u naarad-sentinel -f"
echo
echo "🎉 Naarad Sentinel is now running as a system service!"

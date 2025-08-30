# Naarad Sentinel - Complete Setup & User Guide

## 📖 Table of Contents
1. [What is Naarad Sentinel?](#what-is-naarad-sentinel)
2. [How it Works with Naarad](#how-it-works-with-naarad)
3. [Installation](#installation)
4. [Configuration](#configuration)
5. [Usage & Monitoring](#usage--monitoring)
6. [Integration with Naarad Dashboard](#integration-with-naarad-dashboard)
7. [Advanced Usage](#advanced-usage)
8. [Troubleshooting](#troubleshooting)
9. [Development](#development)

---

## 🤖 What is Naarad Sentinel?

**Naarad Sentinel** is a lightweight system monitoring agent that collects device metrics and sends them to your Naarad dashboard. It runs on your servers, Raspberry Pi devices, and local machines to provide real-time infrastructure monitoring.

### Key Features
- 🔥 **CPU Temperature Monitoring** (Linux/Raspberry Pi)
- 💾 **Disk Space Tracking** (All platforms)
- 🖥️ **System Information** (OS, kernel, architecture)
- 📊 **Prometheus Metrics Export** (Port 9101)
- 🚀 **Multi-platform Support** (Linux, macOS, Windows, Raspberry Pi)
- ⚡ **Lightweight** (4MB binary, minimal CPU usage)
- 🔄 **Auto-sync** with Naarad dashboard

---

## 🔗 How it Works with Naarad

### The Naarad Ecosystem
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Naarad Web    │    │  Naarad API     │    │ Your Devices    │
│   Dashboard     │◄───┤   Backend       │◄───┤ Naarad Sentinel │
│                 │    │                 │    │                 │
│ • Domain Mgmt   │    │ • Data Storage  │    │ • CPU Temp      │
│ • WordPress     │    │ • Plan Limits   │    │ • Disk Space    │
│ • Device Metrics│    │ • Authentication│    │ • System Info   │
│ • Alerts        │    │ • Sync Control  │    │ • Prometheus    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Integration Benefits
- **Unified Dashboard**: View all your infrastructure alongside domains and WordPress sites
- **Smart Alerts**: Get notified about disk space, high temps, or system issues
- **Historical Data**: Track performance trends over time
- **Plan-based Access**: Free plan monitors basic metrics, paid plans add advanced features

---

## 🚀 Installation

### 🍓 Raspberry Pi (Recommended)
```bash
# One-command installation
curl -sSL https://raw.githubusercontent.com/inddev/naarad-sentinel/main/install-pi.sh | bash

# With API key
curl -sSL https://raw.githubusercontent.com/inddev/naarad-sentinel/main/install-pi.sh | bash -s YOUR_API_KEY
```

### 🐧 Linux (x64/ARM64)
```bash
# Download latest release
wget https://github.com/inddev/naarad-sentinel/releases/latest/download/naarad-sentinel-linux-x64
chmod +x naarad-sentinel-linux-x64
mv naarad-sentinel-linux-x64 naarad-sentinel

# Setup
./naarad-sentinel --setup --api-key=YOUR_API_KEY
```

### 🍎 macOS (Intel/Apple Silicon)
```bash
# Intel Macs
wget https://github.com/inddev/naarad-sentinel/releases/latest/download/naarad-sentinel-macos-intel
chmod +x naarad-sentinel-macos-intel
mv naarad-sentinel-macos-intel naarad-sentinel

# Apple Silicon Macs  
wget https://github.com/inddev/naarad-sentinel/releases/latest/download/naarad-sentinel-macos-arm
chmod +x naarad-sentinel-macos-arm
mv naarad-sentinel-macos-arm naarad-sentinel

# Setup
./naarad-sentinel --setup --api-key=YOUR_API_KEY
```

### 🪟 Windows
```powershell
# Download from: https://github.com/inddev/naarad-sentinel/releases/latest/download/naarad-sentinel-windows-x64.exe
# Then run:
naarad-sentinel-windows-x64.exe --setup --api-key=YOUR_API_KEY
```

---

## 🔧 Configuration

### Getting Your API Key
1. Log in to your **Naarad Dashboard** at [app.naarad.dev](https://app.naarad.dev)
2. Go to **Settings** → **API Keys** 
3. Click **Generate New Key** → **Device Monitoring**
4. Copy the key (format: `naarad_device_xxxxx...`)

### Interactive Setup
```bash
./naarad-sentinel --setup --api-key=naarad_device_your_key_here
```

**Setup Process:**
1. 🔧 **Device Name**: Auto-generated or custom (e.g., \"raspberry-pi-living-room\")
2. 💾 **Configuration**: Saved to `~/.naarad-sentinel/config.json`
3. ✅ **Validation**: Tests connection to Naarad API
4. 🚀 **Ready**: Device appears in your dashboard

### Configuration File
Located at `~/.naarad-sentinel/config.json`:
```json
{
  \"api_key\": \"naarad_device_xxxxx...\",
  \"device_id\": \"raspberry-pi-living-room\",
  \"device_name\": \"Raspberry Pi - Living Room\",
  \"endpoint\": \"https://app.naarad.dev/api/devices/metrics/ingest\",
  \"interval\": 60,
  \"prometheus_enabled\": true,
  \"prometheus_port\": 9101,
  \"push_enabled\": true
}
```

---

## 📊 Usage & Monitoring

### Starting Monitoring
```bash
# Start monitoring (foreground)
./naarad-sentinel

# Check specific modes
./naarad-sentinel --legacy    # HTTP server only
./naarad-sentinel --dev      # Use localhost endpoint
./naarad-sentinel --interval 30  # Custom interval (seconds)
```

### Installing as Service (Linux/Pi)
```bash
# Use the provided installer
chmod +x install-service.sh
sudo ./install-service.sh

# Manual systemd service
sudo cp naarad-sentinel /usr/local/bin/
sudo systemctl enable naarad-sentinel
sudo systemctl start naarad-sentinel
```

### Prometheus Metrics (Local)
```bash
# View raw metrics
curl http://localhost:9101/metrics

# Example output:
# device_cpu_temperature_celsius 42.5
# device_disk_free_mb 15420
# sentinel_device_info{os_type=\"Linux\", hostname=\"pi4\"} 1
```

---

## 🎛️ Integration with Naarad Dashboard

### Dashboard Features
Once connected, your Naarad dashboard shows:

**📱 Device Overview:**
- All connected devices in one view
- Real-time status (online/offline)
- Quick stats (temp, disk, uptime)

**📈 Monitoring Graphs:**
- CPU temperature trends
- Disk usage over time  
- System health scores

**🚨 Smart Alerts:**
- Disk space warnings (< 10% free)
- High temperature alerts (> 70°C)
- Device offline notifications

**📋 Device Management:**
- Rename devices
- Configure alert thresholds
- View detailed system info

### Plan Features
| Feature | Free | Pro ($5/mo) | Power ($19/mo) |
|---------|------|-------------|----------------|
| Devices | 2 | 10 | Unlimited |
| Metrics History | 7 days | 90 days | 1 year |
| Alerts | Basic | Advanced | Custom |
| API Access | ❌ | ✅ | ✅ |
| Integrations | ❌ | ✅ | ✅ |

---

## 🔧 Advanced Usage

### Development Mode
```bash
# Point to local Naarad instance
./naarad-sentinel --dev --interval 10
```

### Custom Metrics Collection
Modify `config.json` to adjust:
- Collection interval (default: 60 seconds)
- Prometheus port (default: 9101)
- Enable/disable specific metrics

### Docker Deployment
```dockerfile
FROM debian:bullseye-slim
COPY naarad-sentinel /usr/local/bin/
RUN chmod +x /usr/local/bin/naarad-sentinel
CMD [\"/usr/local/bin/naarad-sentinel\"]
```

### Multiple Environments
```bash
# Production
./naarad-sentinel --api-key=prod_key

# Staging  
./naarad-sentinel --dev --api-key=staging_key
```

---

## 🛠️ Troubleshooting

### Common Issues

**❌ \"Configuration file not found\"**
```bash
# Run setup first
./naarad-sentinel --setup --api-key=YOUR_KEY
```

**❌ \"Failed to send metrics\"**
```bash
# Check internet connection
curl https://app.naarad.dev/health

# Check API key
./naarad-sentinel --setup --api-key=NEW_KEY
```

**❌ \"Permission denied on port 9101\"**
```bash
# Use custom port
./naarad-sentinel --prometheus-port 9102
```

**❌ \"CPU temperature not available\"**
- Normal on macOS/Windows
- Linux: Check `/sys/class/thermal/thermal_zone0/temp` exists

### Debug Mode
```bash
# View detailed logs
RUST_LOG=debug ./naarad-sentinel

# Test connectivity
curl -H \"Authorization: Bearer YOUR_API_KEY\" \\
  https://app.naarad.dev/api/devices/test
```

### Getting Help
- 📧 **Support**: Contact through Naarad dashboard
- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/inddev/naarad-sentinel/issues)
- 📖 **Documentation**: Check README.md
- 💬 **Community**: Join Naarad Discord (link in dashboard)

---

## 👨‍💻 Development

### Building from Source
```bash
# Clone repository
git clone https://github.com/inddev/naarad-sentinel.git
cd naarad-sentinel

# Build locally
cargo build --release

# Cross-compile all platforms
./build-all.sh

# Docker-based cross-compilation
./docker-build.sh
```

### Project Structure
```
naarad-sentinel/
├── src/
│   ├── collectors/          # Metric collection modules
│   │   ├── cpu_temp.rs     # CPU temperature (Linux)
│   │   ├── disk_free.rs    # Disk space (all platforms) 
│   │   ├── device_info.rs  # System information
│   │   └── mod.rs
│   ├── config.rs           # Configuration management
│   ├── http_client.rs      # Naarad API communication
│   ├── metrics.rs          # Prometheus formatting
│   └── main.rs             # CLI & main logic
├── .github/workflows/      # Auto-release CI/CD
├── install-pi.sh          # Pi installer script
└── docker-build.sh        # Cross-compilation
```

### Adding New Metrics
1. **Create collector** in `src/collectors/new_metric.rs`
2. **Export function** that returns `Option<f64>` or metric struct  
3. **Add to main.rs** and `format_prometheus_metrics()`
4. **Test on target platforms**
5. **Update documentation**

### Contributing
- 🍴 Fork the repository
- 🌿 Create feature branch (`feature/memory-usage`)
- ✅ Add tests and documentation
- 📤 Submit pull request

---

## 🎉 Getting Started Checklist

- [ ] 📱 **Sign up for Naarad** at [app.naarad.dev](https://app.naarad.dev)  
- [ ] 🔑 **Generate API key** in dashboard settings
- [ ] 📥 **Download Sentinel** for your platform
- [ ] ⚙️ **Run setup** with API key
- [ ] 🚀 **Start monitoring** and verify in dashboard
- [ ] 📊 **Configure alerts** for your needs
- [ ] 🔄 **Install as service** for 24/7 monitoring

**Welcome to the Naarad ecosystem!** 🎊

Your infrastructure monitoring is now unified with your domain management and WordPress oversight in one powerful dashboard.

---

*Need help? Check the [troubleshooting section](#troubleshooting) or contact support through your Naarad dashboard.*

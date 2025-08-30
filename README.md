# Naarad Sentinel

**Naarad Sentinel** is a lightweight system monitoring agent that connects your devices to the **Naarad Dashboard** for unified infrastructure monitoring alongside domain management and WordPress oversight.

> 📖 **[Read the Complete Setup & User Guide →](./GUIDE.md)**

---

## 🚀 Quick Start

### 🍓 Raspberry Pi (One Command)
```bash
curl -sSL https://raw.githubusercontent.com/inddev/naarad-sentinel/main/install-pi.sh | bash
```

### 🐧 Linux / 🍎 macOS / 🪟 Windows
1. **Download** for your platform from [Releases](https://github.com/inddev/naarad-sentinel/releases/latest)
2. **Get API key** from [Naarad Dashboard](https://app.naarad.dev) → Settings → API Keys
3. **Setup**: `./naarad-sentinel --setup --api-key=YOUR_KEY`
4. **Start**: `./naarad-sentinel`

---

## ✨ Features

- 🔥 **CPU Temperature** (Linux/Raspberry Pi)
- 💾 **Disk Space Monitoring** (All platforms)  
- 🖥️ **System Information** (OS, kernel, architecture)
- 📊 **Prometheus Metrics** (`:9101/metrics`)
- 🚀 **Multi-platform** (Linux x64/ARM64, macOS Intel/ARM, Windows, Raspberry Pi)
- ⚡ **Lightweight** (4MB binary, minimal resources)
- 🔄 **Auto-sync** with Naarad dashboard

---

## 🎛️ Dashboard Integration

Once connected, your [Naarad Dashboard](https://app.naarad.dev) shows:
- 📱 **All devices** in unified view
- 📈 **Real-time metrics** and historical trends
- 🚨 **Smart alerts** for disk space, temperature, offline devices
- 🔧 **Device management** and configuration

---

## 📊 Example Output

```bash
curl http://localhost:9101/metrics
```

```
device_cpu_temperature_celsius 42.5
device_disk_free_mb 15420
sentinel_device_info{os_type=\"Linux\", os_version=\"Raspberry Pi OS\", hostname=\"pi4\"} 1
```

---

## 🛠️ Development

### Build from Source
```bash
git clone https://github.com/inddev/naarad-sentinel.git
cd naarad-sentinel
cargo build --release
```

### Cross-Compile All Platforms
```bash
# Local build script
./build-all.sh

# Docker-based (recommended)
./docker-build.sh
```

---

## 📁 Project Structure

```
src/
├── collectors/          # Metric collection (CPU, disk, system info)
├── config.rs           # Configuration management  
├── http_client.rs      # Naarad API communication
├── metrics.rs          # Prometheus formatting
└── main.rs             # CLI interface
```

---

## 🤝 Contributing

1. 🍴 **Fork** the repository
2. 🌿 **Create** feature branch
3. 📊 **Add** new metrics in `src/collectors/`
4. ✅ **Test** on target platforms
5. 📤 **Submit** pull request

---

## 📜 License

MIT License - see [LICENSE](LICENSE) for details.

---

## 🔗 Links

- 🌐 **[Naarad Dashboard](https://app.naarad.dev)** - Main application
- 📖 **[Complete Guide](./GUIDE.md)** - Detailed setup & usage
- 🐛 **[Issues](https://github.com/inddev/naarad-sentinel/issues)** - Bug reports & feature requests
- 📦 **[Releases](https://github.com/inddev/naarad-sentinel/releases)** - Download binaries

---

*Part of the **Naarad Ecosystem** - Unified monitoring for your digital infrastructure.*

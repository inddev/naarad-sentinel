# Naarad Sentinel - Developer Operations Guide

## 📋 Project Overview

**Naarad Sentinel** is the device monitoring component of the Naarad ecosystem. It's a lightweight Rust binary that collects system metrics and integrates with the main Naarad dashboard for unified infrastructure monitoring.

### Purpose in Naarad Ecosystem
- **Extends Naarad beyond domains** - Adds server/device monitoring to domain + WordPress management
- **Unified dashboard** - All infrastructure in one place
- **Plan differentiation** - Free plan gets basic monitoring, paid plans get advanced features
- **Cross-platform** - Works on user's servers, Pi devices, local machines

---

## 🏗️ Architecture

### Technical Stack
- **Language**: Rust (performance, cross-platform, single binary)
- **HTTP**: Hyper for async server
- **System Info**: sysinfo + platform-specific APIs  
- **Configuration**: JSON config in user home directory
- **Build**: GitHub Actions for automated cross-compilation

### Data Flow
```
Device → Sentinel → Naarad API → Dashboard
     ↓
Prometheus (:9101) ← Grafana/Other tools
```

### Components
```
src/
├── collectors/          # Platform-specific metric collection
│   ├── cpu_temp.rs     # Linux thermal zones
│   ├── disk_free.rs    # Cross-platform disk space
│   ├── device_info.rs  # OS/kernel/hostname detection
│   └── mod.rs
├── config.rs           # Setup & configuration management
├── http_client.rs      # Naarad API communication
├── metrics.rs          # Prometheus format output
└── main.rs             # CLI interface & main loop
```

---

## 🔑 Configuration & Secrets

### API Keys
**Format**: `naarad_device_[64-char-hex]`
**Generated**: In main Naarad dashboard → Settings → API Keys → Device Monitoring
**Scope**: Device metrics submission only (not user data access)

### Configuration Storage
**Location**: `~/.naarad-sentinel/config.json`
**Permissions**: User-readable only
**Content**:
```json
{
  \"api_key\": \"naarad_device_xxxxx...\",
  \"device_id\": \"macos-johns-macbook\", 
  \"device_name\": \"John's MacBook\",
  \"endpoint\": \"https://app.naarad.dev/api/devices/metrics/ingest\",
  \"interval\": 60,
  \"prometheus_enabled\": true,
  \"prometheus_port\": 9101,
  \"push_enabled\": true
}
```

### Environment Variables
```bash
# Development mode
RUST_LOG=debug              # Enable debug logging
NAARAD_ENDPOINT=localhost   # Override API endpoint
```

---

## 🚢 Deployment & Distribution

### Release Strategy
**Primary**: GitHub Releases with auto-built binaries
**Automation**: GitHub Actions on tag push

### Supported Platforms
| Platform | Target | Binary Name | Notes |
|----------|---------|-------------|-------|
| Linux x64 | `x86_64-unknown-linux-gnu` | `naarad-sentinel-linux-x64` | Most servers |
| Linux ARM64 | `aarch64-unknown-linux-gnu` | `naarad-sentinel-linux-arm64` | Pi 4 64-bit |
| Raspberry Pi | `armv7-unknown-linux-gnueabihf` | `naarad-sentinel-raspberry-pi` | Pi 3/4 32-bit |
| macOS Intel | `x86_64-apple-darwin` | `naarad-sentinel-macos-intel` | Intel Macs |
| macOS ARM | `aarch64-apple-darwin` | `naarad-sentinel-macos-arm` | M1/M2 Macs |
| Windows x64 | `x86_64-pc-windows-gnu` | `naarad-sentinel-windows-x64.exe` | Windows 10+ |

### Build Process
```bash
# Local development
cargo build --release

# Cross-compilation (Docker)
./docker-build.sh

# Local cross-compilation 
./build-all.sh

# Release (triggers GitHub Actions)
git tag v0.1.1
git push origin v0.1.1
```

### Distribution Methods
1. **GitHub Releases** - Primary distribution
2. **One-line Pi installer** - `curl ... | bash`
3. **Direct download** - Platform-specific binaries
4. **Future**: Package managers (APT, Homebrew, Chocolatey)

---

## 🔄 Development Workflow

### Local Development
```bash
# Setup development environment
git clone https://github.com/inddev/naarad-sentinel.git
cd naarad-sentinel

# Install Rust (if needed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Development build
cargo run -- --dev --api-key=test_key

# Test metrics endpoint
curl http://localhost:9101/metrics
```

### Testing Pipeline
```bash
# 1. Syntax check
cargo check

# 2. Local build test
cargo build --release

# 3. Cross-platform test (Docker)
./docker-test.sh

# 4. Full cross-compilation
./docker-build.sh

# 5. Final verification
./final-test.sh
```

### Code Quality
- **Linting**: `cargo clippy`
- **Formatting**: `cargo fmt`
- **Testing**: `cargo test`
- **Dependencies**: Regular `cargo audit` for security

---

## 🔌 Integration Points

### Naarad Main App
**API Endpoint**: `POST /api/devices/metrics/ingest`
**Payload**:
```json
{
  \"device_id\": \"macos-johns-macbook\",
  \"device_name\": \"John's MacBook\", 
  \"device_type\": \"macOS\",
  \"timestamp\": \"2025-08-30T14:30:00Z\",
  \"metrics\": {
    \"device_cpu_temperature_celsius\": 45.2,
    \"device_disk_free_mb\": 128000,
    \"sentinel_device_info\": {
      \"os_type\": \"macOS\",
      \"os_version\": \"14.1\",
      \"kernel\": \"23.1.0\",
      \"arch\": \"arm64\",
      \"hostname\": \"Johns-MacBook\"
    }
  }
}
```

### Authentication
**Method**: Bearer token in Authorization header
**Token**: API key from Naarad dashboard
**Validation**: Backend checks API key scope and user limits

### Dashboard Features
**Device List**: Shows all connected devices with status
**Metrics Graphs**: Historical CPU temp, disk usage trends  
**Alerts**: Configurable thresholds for temperature, disk space
**Plan Limits**: Free (2 devices), Pro (10 devices), Power (unlimited)

---

## 📊 Monitoring & Alerts

### Metrics Collected
| Metric | Linux | macOS | Windows | Raspberry Pi |
|--------|-------|-------|---------|--------------|
| CPU Temperature | ✅ | ❌ | ❌ | ✅ |
| Disk Free Space | ✅ | ✅ | ✅ | ✅ |
| System Info | ✅ | ✅ | ✅ | ✅ |
| Memory Usage | 🔄 | 🔄 | 🔄 | 🔄 |
| Network Stats | 🔄 | 🔄 | 🔄 | 🔄 |

### Alert Thresholds (Default)
- **Disk Space**: Warning at 15% free, Critical at 5%
- **CPU Temperature**: Warning at 70°C, Critical at 85°C  
- **Device Offline**: Alert after 5 minutes

### Prometheus Integration
**Endpoint**: `http://device:9101/metrics`
**Format**: Standard Prometheus text exposition
**Usage**: Can be scraped by Grafana, Prometheus, etc.

---

## 🚨 Operational Considerations

### Resource Usage
- **Binary Size**: ~4MB (statically linked)
- **Memory**: <10MB RSS during operation
- **CPU**: <1% on modern hardware
- **Network**: ~1KB per minute (metrics push)

### Security
- **API Keys**: Scoped to device metrics only
- **Network**: HTTPS for all API communication
- **Local Storage**: Config file readable by user only
- **No Root Required**: Runs as regular user

### Reliability
- **Graceful Failures**: Continues running if API temporarily unavailable
- **Retry Logic**: Exponential backoff for failed requests
- **Service Management**: Systemd integration for auto-restart
- **Logging**: Structured logs for debugging

---

## 🐛 Known Issues & Workarounds

### Platform-Specific
**macOS**: CPU temperature not available (hardware limitation)
**Windows**: Some system paths may need elevated permissions
**Linux**: Thermal zone detection varies by hardware

### Network Issues
**Corporate Firewalls**: May block API calls (whitelist app.naarad.dev)
**Dynamic IPs**: Device may appear as new device after IP change

### Resource Constraints
**Raspberry Pi Zero**: May need increased interval (120+ seconds)
**Low Disk**: Sentinel stops metrics collection at <100MB free

---

## 🔮 Roadmap

### Next Release (v0.2.0)
- [ ] Memory usage collection
- [ ] Network interface statistics
- [ ] Custom metric intervals per type
- [ ] Configuration via environment variables
- [ ] Service installer improvements

### Future Versions
- [ ] Plugin system for custom metrics
- [ ] Local caching for offline operation  
- [ ] Real-time alerting (WebSocket)
- [ ] Performance profiling tools
- [ ] Container-aware metrics

---

## 🧰 Useful Commands

### Development
```bash
# Quick test all platforms
./final-test.sh

# Debug specific platform
RUST_LOG=debug cargo run -- --dev

# Cross-compile single platform
cargo build --release --target armv7-unknown-linux-gnueabihf
```

### Deployment
```bash
# Update existing installation
./naarad-sentinel --setup --api-key=NEW_KEY

# Check service status (Linux/Pi)
sudo systemctl status naarad-sentinel

# View real-time logs
sudo journalctl -u naarad-sentinel -f
```

### Maintenance
```bash
# Check binary size
ls -lh target/release/naarad-sentinel

# Verify metrics endpoint
curl -s http://localhost:9101/metrics | head -10

# Test API connectivity
curl -H \"Authorization: Bearer YOUR_API_KEY\" \\
  https://app.naarad.dev/api/devices/test
```

---

## 📞 Support & Escalation

### User Support
1. **First**: Check troubleshooting in GUIDE.md
2. **Dashboard**: Use support chat in Naarad dashboard  
3. **GitHub**: Issues for bugs/feature requests
4. **Email**: Support ticket for urgent issues

### Developer Escalation
1. **Compilation Issues**: Check platform-specific dependencies
2. **API Changes**: Coordinate with main Naarad backend team
3. **Release Issues**: GitHub Actions logs and artifact inspection
4. **Performance**: Profile with `cargo flamegraph`

---

## 📈 Success Metrics

### Technical KPIs
- **Uptime**: >99.9% device availability
- **Resource Usage**: <10MB RAM, <1% CPU
- **Data Delivery**: <5% failed metric submissions
- **Cross-platform**: All 6 platforms building successfully

### Business KPIs  
- **Adoption**: Device monitoring increases Naarad plan upgrades
- **Retention**: Users with devices have higher retention
- **Support Load**: <5% of device monitoring users need support

---

*This guide covers everything needed to develop, deploy, and operate Naarad Sentinel as part of the broader Naarad ecosystem.*

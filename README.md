# Naarad Sentinel

**Naarad Sentinel** is a lightweight system metrics exporter written in Rust. It runs on Linux, macOS, and Windows — with special support for Raspberry Pi — and exposes system data in **Prometheus format** on `:9101/metrics`.

---

## 🚀 Features

* CPU temperature (Linux / Raspberry Pi)
* Disk free space (Linux, macOS, Windows)
* Device info (OS type, version, kernel, architecture, hostname)
* Tiny footprint, async HTTP server
* Prometheus-compatible text exposition

---

## 📦 Getting Started

### 1) Run locally (macOS/Linux/Windows)

```bash
# Build & run
cargo build --release
./target/release/naarad-sentinel

# Test the metrics endpoint
curl http://localhost:9101/metrics
```

### 2) Cross-compile for Raspberry Pi (armv7)

```bash
# (Recommended) zig-based cross build
cargo install cargo-zigbuild
rustup target add armv7-unknown-linux-gnueabihf
cargo zigbuild --release --target armv7-unknown-linux-gnueabihf

# Copy to Pi
scp target/armv7-unknown-linux-gnueabihf/release/naarad-sentinel \
    <user>@<pi-host>:/home/<user>/

# Run on the Pi
ssh <user>@<pi-host> "./naarad-sentinel"
# Then from your machine:
curl http://<pi-host>:9101/metrics
```

> Tip: Replace `<user>` and `<pi-host>` with your actual SSH username/host (e.g., `pi@pi.local`).

---

## 🔁 Optional: Run as a systemd service (Pi/Linux)

```bash
# Copy the binary to a standard location
sudo mv ~/naarad-sentinel /usr/local/bin/naarad-sentinel
sudo chmod +x /usr/local/bin/naarad-sentinel

# Create the service
sudo tee /etc/systemd/system/naarad-sentinel.service >/dev/null <<'EOF'
[Unit]
Description=Naarad Sentinel
After=network.target

[Service]
User=<user>
ExecStart=/usr/local/bin/naarad-sentinel
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
EOF

# Enable & start
sudo systemctl daemon-reload
sudo systemctl enable naarad-sentinel
sudo systemctl start naarad-sentinel
sudo systemctl status naarad-sentinel --no-pager
```

---

## 📊 Example Output

```text
device_cpu_temperature_celsius 44.388
device_disk_free_mb 26449
sentinel_device_info{os_type="Linux", os_version="Raspbian GNU/Linux 12 (bookworm)", kernel="6.12.34+rpt-rpi-v7", arch="armv7l", hostname="p2"} 1
```

---

## 📈 Prometheus integration

```yaml
scrape_configs:
  - job_name: 'naarad_sentinel'
    static_configs:
      - targets:
          - 'p2.local:9101'   # your Raspberry Pi
          # - 'another-host:9101'
```

---

## 🛠 Development Guide

### Requirements

* Rust (stable), Cargo, Git
* (Optional) `cargo-zigbuild` for cross-compiling to Pi from macOS
* Linux-only metrics (e.g. CPU temperature) gracefully omit on non-Linux hosts

### Repo layout

```text
src/
  collectors/
    cpu_temp.rs      # CPU temperature (Linux/Pi)
    disk_free.rs     # Per-OS disk free handling (Linux/macOS/Windows)
    device_info.rs   # Friendly OS name/version, kernel, arch, hostname
    mod.rs
  main.rs            # HTTP server + /metrics assembly
```

### Workflow

```bash
# 1) Fork & clone
git clone git@github.com:<you>/naarad-sentinel.git
cd naarad-sentinel

# 2) Create a feature branch
git checkout -b feature/<short-name>

# 3) Run locally
cargo run

# 4) Test the endpoint
curl http://localhost:9101/metrics

# 5) Commit & push
git add .
git commit -m "feat: <what you changed>"
git push origin feature/<short-name>

# 6) Open a PR on GitHub
```

---

## 🧭 OS Notes

* **CPU temperature**: reported on Linux/Pi via `/sys/class/thermal/thermal_zone0/temp`. Not shown on macOS/Windows.
* **Disk free**:

  * Linux: via `statvfs()` (actual free).
  * macOS: via `df -m /` to avoid APFS “purgeable” illusions.
  * Windows: via `GetDiskFreeSpaceExW`.
* **Device info**: single metric `sentinel_device_info{...} 1` encodes OS details as labels (Prometheus-style).

---

## 🗺 Roadmap

* systemd unit in-repo (packaged)
* Memory usage & CPU usage collectors
* Network interface stats
* Grafana dashboard templates
* Docker image
* Configurable collectors & port (env/flags)

---

## 🤝 Contributing

Contributions welcome! Guidelines:

* Place new metrics under `src/collectors/`.
* Use Prometheus naming conventions (unit suffixes like `_celsius`, `_bytes`, `_seconds`).
* Prefer label-encoded info metrics (e.g., `*_info{...} 1`) over string values.
* Handle OS differences with `#[cfg(target_os = "...")]` so outputs stay correct.
* Include an example of your metric in the PR description.

---

## 📜 License

MIT — see `LICENSE` for details.

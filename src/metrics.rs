// src/metrics.rs
use crate::collectors::{cpu_temp, disk_free, device_info};

pub fn format_prometheus_metrics() -> String {
    let mut metrics = String::new();

    // CPU temp
    if let Some(temp) = cpu_temp::get_cpu_temp_celsius() {
        metrics.push_str(&format!("device_cpu_temperature_celsius {}\n", temp));
    }

    // Disk free
    if let Some(free) = disk_free::get_disk_free_mb() {
        metrics.push_str(&format!("device_disk_free_mb {}\n", free));
    }

    // Device info
    let info = device_info::get_device_info();
    metrics.push_str(&format!(
        "sentinel_device_info{{os_type=\"{}\", os_version=\"{}\", kernel=\"{}\", arch=\"{}\", hostname=\"{}\"}} 1\n",
        info.os_type, info.os_version, info.kernel_version, info.architecture, info.hostname
    ));

    metrics
}

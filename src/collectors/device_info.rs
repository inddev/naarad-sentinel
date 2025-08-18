// src/collectors/device_info.rs
#[derive(Debug)]
pub struct DeviceInfo {
    pub os_type: String,
    pub os_version: String,
    pub kernel_version: String,
    pub architecture: String,
    pub hostname: String,
}

//
// macOS
//
#[cfg(target_os = "macos")]
pub fn get_device_info() -> DeviceInfo {
    use std::process::Command;
    use uname::uname;

    let os_type = "macOS".to_string();

    // sw_vers gives user-friendly version
    let os_version = Command::new("sw_vers")
        .arg("-productVersion")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "Unknown".to_string())
        .trim()
        .to_string();

    let kernel_version = uname()
        .map(|u| u.release)
        .unwrap_or_else(|_| "Unknown".to_string());

    let architecture = uname()
        .map(|u| u.machine)
        .unwrap_or_else(|_| "Unknown".to_string());

    let hostname = uname()
        .map(|u| u.nodename)
        .unwrap_or_else(|_| "Unknown".to_string());

    DeviceInfo { os_type, os_version, kernel_version, architecture, hostname }
}

//
// Linux
//
#[cfg(target_os = "linux")]
pub fn get_device_info() -> DeviceInfo {
    use uname::uname;

    let os_type = "Linux".to_string();

    // Try /etc/os-release for pretty name
    let os_version = std::fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|c| {
            c.lines()
                .find(|l| l.starts_with("PRETTY_NAME="))
                .map(|l| l.trim_start_matches("PRETTY_NAME=").trim_matches('"').to_string())
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let kernel_version = uname()
        .map(|u| u.release)
        .unwrap_or_else(|_| "Unknown".to_string());

    let architecture = uname()
        .map(|u| u.machine)
        .unwrap_or_else(|_| "Unknown".to_string());

    let hostname = uname()
        .map(|u| u.nodename)
        .unwrap_or_else(|_| "Unknown".to_string());

    DeviceInfo { os_type, os_version, kernel_version, architecture, hostname }
}

//
// Windows
//
#[cfg(target_os = "windows")]
pub fn get_device_info() -> DeviceInfo {
    use std::process::Command;
    use uname::uname;

    let os_type = "Windows".to_string();

    // Windows build/version via `ver` command
    let os_version = Command::new("cmd")
        .args(["/C", "ver"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .unwrap_or_else(|| "Unknown".to_string())
        .trim()
        .to_string();

    let kernel_version = uname()
        .map(|u| u.release)
        .unwrap_or_else(|_| "Unknown".to_string());

    let architecture = uname()
        .map(|u| u.machine)
        .unwrap_or_else(|_| "Unknown".to_string());

    let hostname = uname()
        .map(|u| u.nodename)
        .unwrap_or_else(|_| "Unknown".to_string());

    DeviceInfo { os_type, os_version, kernel_version, architecture, hostname }
}

// src/config.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::fs;
use std::io::{self, Write};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub device_id: String,
    pub device_name: String,
    pub endpoint: String,
    pub interval: u64,
    
    // New options
    pub prometheus_enabled: bool,
    pub prometheus_port: u16,
    pub push_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            device_id: String::new(),
            device_name: String::new(),
            endpoint: "https://app.naarad.dev/api/devices/metrics/ingest".to_string(),
            interval: 60, // 60 seconds default
            prometheus_enabled: true,
            prometheus_port: 9101,
            push_enabled: true,
        }
    }
}

pub fn config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?;
    
    let mut path = home_dir;
    path.push(".naarad-sentinel");
    
    // Create directory if it doesn't exist
    fs::create_dir_all(&path)?;
    
    path.push("config.json");
    Ok(path)
}

pub fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let config_file = config_path()?;
    let json = serde_json::to_string_pretty(config)?;
    fs::write(config_file, json)?;
    Ok(())
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_file = config_path()?;
    
    if !config_file.exists() {
        return Err("Configuration file not found. Please run with --setup first.".into());
    }
    
    let content = fs::read_to_string(config_file)?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn generate_device_name() -> String {
    let hostname = get_hostname();
    let os_type = std::env::consts::OS;
    format!("{}-{}", os_type, hostname)
}

#[cfg(unix)]
fn get_hostname() -> String {
    uname::uname()
        .map(|info| info.nodename)
        .unwrap_or_else(|_| "unknown-device".to_string())
}

#[cfg(windows)]
fn get_hostname() -> String {
    std::env::var("COMPUTERNAME")
        .unwrap_or_else(|_| "windows-pc".to_string())
        .to_lowercase() // Make it consistent
}

pub fn interactive_setup(api_key: String) -> Result<Config, Box<dyn std::error::Error>> {
    println!("🔧 Naarad Sentinel Setup");
    println!("========================");
    println!();
    
    print!("Enter a name for this device (or press Enter to auto-generate): ");
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let device_name = if input.trim().is_empty() {
        let generated = generate_device_name();
        println!("Generated device name: {}", generated);
        generated
    } else {
        input.trim().to_string()
    };
    
    // Generate device ID from name (lowercase, replace spaces with dashes)
    let device_id = device_name
        .to_lowercase()
        .replace(" ", "-")
        .replace("_", "-");
    
    println!();
    println!("Configuration Summary:");
    println!("  Device Name: {}", device_name);
    println!("  Device ID: {}", device_id);
    println!("  API Key: {}***", &api_key[..std::cmp::min(api_key.len(), 20)]);
    println!();
    
    let config = Config {
        api_key,
        device_id,
        device_name: device_name.clone(),
        endpoint: if cfg!(debug_assertions) {
            "http://localhost:5001/api/devices/metrics/ingest".to_string()
        } else {
            "https://app.naarad.dev/api/devices/metrics/ingest".to_string()
        },
        interval: 60,
        prometheus_enabled: true,
        prometheus_port: 9101,
        push_enabled: true,
    };
    
    save_config(&config)?;
    
    let config_path = config_path()?;
    println!("✅ Configuration saved to: {}", config_path.display());
    println!("📡 Device '{}' is ready to start monitoring!", device_name);
    println!();
    println!("To start monitoring, run: ./naarad-sentinel");
    
    Ok(config)
}

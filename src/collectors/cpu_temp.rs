// src/collectors/cpu_temp.rs
use std::fs;

pub fn get_cpu_temp_celsius() -> Option<f64> {
    let path = "/sys/class/thermal/thermal_zone0/temp";
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(millideg) = content.trim().parse::<f64>() {
            return Some(millideg / 1000.0);
        }
    }
    None
}

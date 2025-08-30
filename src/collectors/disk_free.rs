// src/collectors/disk_free.rs
#[cfg(target_os = "macos")]
pub fn get_disk_free_mb() -> Option<u64> {
    // Use macOS-specific APIs (statvfs lies on APFS)
    use std::process::Command;
    if let Ok(output) = Command::new("df").arg("-m").arg("/").output() {
        if let Ok(text) = String::from_utf8(output.stdout) {
            if let Some(line) = text.lines().nth(1) {
                let cols: Vec<&str> = line.split_whitespace().collect();
                if cols.len() >= 4 {
                    if let Ok(free_mb) = cols[3].parse::<u64>() {
                        return Some(free_mb);
                    }
                }
            }
        }
    }
    None
}

#[cfg(target_os = "linux")]
pub fn get_disk_free_mb() -> Option<u64> {
    use nix::sys::statvfs::statvfs;
    use std::path::Path;
    if let Ok(stats) = statvfs(Path::new("/")) {
        return Some((stats.blocks_free() as u64 * stats.block_size() as u64) / 1024 / 1024);
    }
    None
}

#[cfg(target_os = "windows")]
pub fn get_disk_free_mb() -> Option<u64> {
    use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
    use windows::core::PCWSTR;

    let drive: Vec<u16> = "C:\\\0".encode_utf16().collect();
    let mut free_bytes_available: u64 = 0;
    let mut total_number_of_bytes: u64 = 0;
    let mut total_number_of_free_bytes: u64 = 0;

    unsafe {
        if GetDiskFreeSpaceExW(
            PCWSTR::from_raw(drive.as_ptr()),
            Some(&mut free_bytes_available),
            Some(&mut total_number_of_bytes),
            Some(&mut total_number_of_free_bytes),
        )
        .is_ok()
        {
            return Some(free_bytes_available / 1024 / 1024);
        }
    }
    None
}

use std::process::Command;

#[link(name = "lingaalib", kind = "static")]
unsafe extern "C" {
    fn CountPhysicalDriveSectors();
}

/// Safe wrapper for the C function
pub fn count_physical_drive_sectors() {
    unsafe { CountPhysicalDriveSectors() }
}

/// Check if the current process has admin rights (Rust implementation)
pub fn is_running_as_admin() -> bool {
    let output = Command::new("net")
        .arg("session")
        .output();

    match output {
        Ok(o) => o.status.success(),
        Err(_) => false,
    }
}


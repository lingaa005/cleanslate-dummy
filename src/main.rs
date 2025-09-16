use rust_lingaa::{is_running_as_admin, count_physical_drive_sectors};
use windows::{

    Win32::Foundation::{HWND},
    Win32::UI::Shell::{ ShellExecuteW,},
    Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL,
};
use windows::core::HSTRING;
use std::io;

pub fn relaunch_elevated() {
    // Get the path to the current executable
    let exe_path = std::env::current_exe().expect("Failed to get current executable path");
    let exe_path_hstring = HSTRING::from(exe_path.as_os_str());
    let verb_hstring = HSTRING::from("runas");

    unsafe {
        let result = ShellExecuteW(
            Some(HWND::default()),
            &verb_hstring,
            &exe_path_hstring,
            None,
            None,
            SW_SHOWNORMAL,
        );

        // Cast the pointer to an integer (isize) before comparing
        if result.0 as isize <= 32 {
            // Also cast the pointer to an integer before printing
            panic!("ShellExecuteW failed with error code: {}", result.0 as isize);
        }
    }

    std::process::exit(0);
}


fn main() {
    if !is_running_as_admin() {
        println!("Not running as admin. Requesting elevation...");
        relaunch_elevated(); // exits current process
    }

    println!("Running as admin. Counting physical drive sectors...");
    count_physical_drive_sectors();
    let mut user_input = String::new();

    // Read a line from standard input and store it in our string
    let _ = io::stdin().read_line(&mut user_input);
}

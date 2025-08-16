use std::io;
use windows::Win32::{
    Foundation::{HINSTANCE, LPARAM, LRESULT, WPARAM, HMODULE},
    UI::WindowsAndMessaging::{
        CallNextHookEx, GetForegroundWindow, PostMessageW, SetWindowsHookExW, 
        HHOOK, KBDLLHOOKSTRUCT, WINDOWS_HOOK_ID
    },
};

use crate::keys::{
    alt_pressed, shift_pressed, update_modifiers, 
    win_pressed, KeyEvent, VK_C, WM_KEYDOWN, WM_SYSKEYDOWN
};

const WM_CLOSE: u32 = 0x0010;

unsafe extern "system" fn keyboard_proc(
    n_code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
    if n_code >= 0 {
        let kbd = *(l_param.0 as *const KBDLLHOOKSTRUCT);
        let vk = kbd.vkCode;
        let key_down = matches!(w_param.0 as u32, WM_KEYDOWN | WM_SYSKEYDOWN);

        update_modifiers(vk, key_down);

        let event = KeyEvent {
            vk_code: vk,
            win: win_pressed(),
            alt: alt_pressed(),
            shift: shift_pressed(),
        };

        #[cfg(debug_assertions)]
        println!(
            "[DEBUG] vk: 0x{:X} down={} win={} alt={} shift={}",
            event.vk_code,
            key_down,
            event.win,
            event.alt,
            event.shift
        );

        if key_down && (event.win || event.alt) && event.shift && event.vk_code == VK_C {
            #[cfg(debug_assertions)]
            println!("[DEBUG] Closing out window now");

            let window = GetForegroundWindow();
            if !window.is_invalid() {
                PostMessageW(Some(window), WM_CLOSE, WPARAM(0), LPARAM(0));
            }
        }
    }

    CallNextHookEx(None, n_code, w_param, l_param)
}

pub fn load_hook(hinst: HMODULE) -> io::Result<HHOOK> {
    const WH_KEYBOARD_LL: WINDOWS_HOOK_ID = WINDOWS_HOOK_ID(13);
    let hmod = HINSTANCE(hinst.0);
    let flags = 0;

    unsafe {
        let result = SetWindowsHookExW(
            WH_KEYBOARD_LL, 
            Some(keyboard_proc),
            Some(hmod),
            flags
        )?;

        Ok(result)
    }
}

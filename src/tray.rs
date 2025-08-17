use std::{ffi::OsStr, io, mem::zeroed, os::windows::ffi::OsStrExt, thread};

use windows::{
    core::{w, Error, PCWSTR}, Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM}, System::LibraryLoader::GetModuleHandleW, 
        UI::{
            Shell::{Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NIM_DELETE, NOTIFYICONDATAW}, 
            WindowsAndMessaging::{
                AppendMenuW, CreatePopupMenu, CreateWindowExW, DefWindowProcW, DestroyMenu, DestroyWindow, 
                DispatchMessageW, GetCursorPos, GetMessageW, LoadImageW, PostMessageW, RegisterClassW, 
                SetForegroundWindow, TrackPopupMenu, TranslateMessage, HICON, HWND_MESSAGE, IMAGE_ICON, 
                LR_DEFAULTSIZE, MF_STRING, MSG, TPM_LEFTALIGN, TPM_RETURNCMD, TPM_RIGHTBUTTON, 
                WINDOW_EX_STYLE, WINDOW_STYLE, WM_LBUTTONUP, WM_NULL, WM_RBUTTONUP, WM_USER, WNDCLASSW
            }
        }
    }
};

const WM_TRAYICON: u32 = WM_USER + 1;
const ID_EXIT: usize = 1; 

pub fn start_tray(tooltip: &str) {
    let tip = tooltip.to_string();

    thread::spawn(move || {
        let icon = load_icon_from_binary().expect("load icon failed");
        if let Err(e) = unsafe { run_tray(icon, &tip) } {
            eprintln!("[tray] run_tray error: {:?}", e);
        }
    });
}

fn load_icon_from_binary() -> io::Result<HICON> {
    let hist = unsafe { GetModuleHandleW(None)? };
    if hist.0.is_null() {
        return Err(io::Error::last_os_error());
    }

    let id = 1u16 as *const u16;

    let hicon = unsafe {
        LoadImageW(
            Some(HINSTANCE(hist.0)),
            PCWSTR(id),
            IMAGE_ICON,
            0,
            0,
            LR_DEFAULTSIZE,
        )
    }?;

    Ok(HICON(hicon.0))
}

unsafe fn run_tray(icon: HICON, tooltip: &str) -> io::Result<()> {
    let hist = GetModuleHandleW(None)?;
    let class_name = w!("QuickCloseWindow");

    let wc = WNDCLASSW {
        lpfnWndProc: Some(wnd_proc),
        hInstance: hist.into(),
        lpszClassName: class_name,
        ..Default::default()
    };

    if RegisterClassW(&wc) == 0 {
        return Err(io::Error::last_os_error());
    }

    let hwnd = CreateWindowExW(
        WINDOW_EX_STYLE(0),
        class_name,
        w!(""),
        WINDOW_STYLE(0),
        0, 0, 0, 0,
        Some(HWND_MESSAGE), 
        None,
        Some(HINSTANCE(hist.0)),
        None,
    )?;

    if hwnd.is_invalid() {
        return Err(io::Error::last_os_error());
    }

    let mut nid: NOTIFYICONDATAW = zeroed();
    nid.cbSize = std::mem::size_of::<NOTIFYICONDATAW>() as u32;
    nid.hWnd = hwnd;
    nid.uID = 1;
    nid.uFlags = NIF_MESSAGE | NIF_ICON | NIF_TIP;
    nid.uCallbackMessage = WM_TRAYICON;
    nid.hIcon = icon;

    let mut tip_w: [u16; 128] = [0; 128];
    let mut len = 0usize;
    for (i, u) in tooltip.encode_utf16().enumerate() {
        if i >= tip_w.len() - 1 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Tooltip string too large for buffer",
            ));
        }
        tip_w[i] = u;
        len += 1;
    }
    nid.szTip[..len + 1].copy_from_slice(&tip_w[..len + 1]);

    if !Shell_NotifyIconW(NIM_ADD, &mut nid).as_bool() {
        return Err(io::Error::last_os_error());
    }

    let mut msg: MSG = zeroed();
    while GetMessageW(&mut msg, None, 0, 0) != false {
        TranslateMessage(&msg);
        DispatchMessageW(&msg);
    }

    Shell_NotifyIconW(NIM_DELETE, &mut nid);

    Ok(())
}

extern "system" fn wnd_proc(
    hwnd: HWND, 
    msg: u32, 
    _wparam: WPARAM, 
    lparam: LPARAM
) -> LRESULT {
    if msg == WM_TRAYICON {
        let ev = lparam.0 as u32;
        if ev == WM_RBUTTONUP || ev == WM_LBUTTONUP {
            unsafe { show_tray_menu(hwnd) };
            return LRESULT(0);
        }
    }
    unsafe { DefWindowProcW(hwnd, msg, WPARAM(0), lparam) }
}

unsafe fn show_tray_menu(hwnd: HWND) -> io::Result<()> {
    let hmenu = CreatePopupMenu()?;
    if hmenu.is_invalid() {
        return Err(io::Error::last_os_error());
    }

    AppendMenuW(hmenu, MF_STRING, ID_EXIT, w!("Exit"));

    let mut pt = POINT::default();
    GetCursorPos(&mut pt);
    SetForegroundWindow(hwnd);

    let cmd = TrackPopupMenu(
        hmenu,
        TPM_LEFTALIGN | TPM_RIGHTBUTTON | TPM_RETURNCMD,
        pt.x,
        pt.y,
        Some(0),
        hwnd,
        None,
    );

    PostMessageW(Some(hwnd), WM_NULL, WPARAM(0), LPARAM(0));

    if cmd.0 == ID_EXIT as i32 {
        DestroyMenu(hmenu);
        DestroyWindow(hwnd);
        std::process::exit(0);
    }

    Ok(())
}


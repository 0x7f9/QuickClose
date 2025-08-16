use std::sync::atomic::{AtomicBool, Ordering};

pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_SYSKEYDOWN: u32 = 0x0104;

pub const VK_C: u32 = 0x43;
const VK_WIN: u32 = 0x5B;
const VK_LALT: u32 = 0xA4;
const VK_SHIFT: u32 = 0xA0;

static WIN_DOWN: AtomicBool = AtomicBool::new(false);
static ALT_DOWN: AtomicBool = AtomicBool::new(false);
static SHIFT_DOWN: AtomicBool = AtomicBool::new(false);

#[derive(Debug)]
pub struct KeyEvent {
    pub vk_code: u32,
    pub win: bool,
    pub alt: bool,
    pub shift: bool
}

#[inline(always)]
pub fn win_pressed() -> bool {
    WIN_DOWN.load(Ordering::SeqCst)
}

#[inline(always)]
pub fn alt_pressed() -> bool {
    ALT_DOWN.load(Ordering::SeqCst)
}

#[inline(always)]
pub fn shift_pressed() -> bool {
    SHIFT_DOWN.load(Ordering::SeqCst)
}

#[inline(always)]
pub fn update_modifiers(vk_code: u32, key_down: bool) {
    match vk_code {
        VK_WIN => WIN_DOWN.store(key_down, Ordering::SeqCst),
        VK_LALT => ALT_DOWN.store(key_down, Ordering::SeqCst),
        VK_SHIFT => SHIFT_DOWN.store(key_down, Ordering::SeqCst),
        _ => {}
    }
}


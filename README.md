## Why QuickClose Exists

I prefer the way tiling window managers on Linux handle closing windows. On Windows, there’s no equivalent unless you break your hand with `Alt+F4` or grab the mouse.

## Overview

QuickClose lets you gracefully close the currently focused application using a key combination paired with a modifier key.

By default, it supports **Windows Key + Shift + C** and **Alt + Shift + C**, so you can pick the combo that fits your keyboard style. The program runs silently in the background and will automatically add itself to your Startup folder, so it launches when Windows starts.

### Features:

- Gracefully closes the focused window using `WM_CLOSE`, ensuring applications prompt for unsaved work
- Minimal, lightweight, and efficient, using only the official Microsoft `windows` crate
- Relies on Windows low level keyboard hook `WH_KEYBOARD_LL` to detect key combo
- System tray icon with a left/right click Exit menu for easy application termination

## Getting Started

### Requirements
- Windows 10 or 11
- Rust toolchain (if building). See [rust-lang.org](https://www.rust-lang.org/tools/install)

### Pre-built Executable

For users who do not want to build from source:

- Download the latest release from the [Releases page](https://github.com/0x7f9/QuickClose/releases)
- Includes `QuickClose.exe` as a standalone binary

> Note: The executable is unsigned, so Windows may show a SmartScreen warning.

### Build Instructions

```bat
git clone https://github.com/0x7f9/QuickClose.git
cd quickclose

:: build release without console window (tray mode)
cargo build --release --features no_console

:: copy the built executable to current directory
copy target\release\QuickClose.exe .

:: run the app
QuickClose.exe
```

## Usage

Once QuickClose is running press one of the supported key combinations while a window is focused to close it gracefully. The program ensures that only the active window receives the close message, and child processes are unaffected unless explicitly managed by the closed window.

### Default Keybindings:

- Windows Key + Shift + C
- Alt + Shift + C

### Can be extended to:

- Support additional modifier keys
- Integrate into larger window management or automation workflows


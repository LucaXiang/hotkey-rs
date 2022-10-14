#[cfg(target_os = "linux")]
use std::ptr;
#[cfg(target_os = "linux")]
use x11::xlib;
#[cfg(target_os = "linux")]
fn main() {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        let root = xlib::XDefaultRootWindow(display);
        let keycode = xlib::XKeysymToKeycode(display, 65) as i32;
        let result = xlib::XGrabKey(
            display,
            keycode,
            0,
            root,
            0,
            xlib::GrabModeAsync,
            xlib::GrabModeAsync,
        );
        if result == 0 {
            println!("RegisterHokey succeeded");
        } else {
            println!("RegisterHokey failed");
        }
    }
}

#[cfg(not(target_os = "macos"))]
fn main() {}

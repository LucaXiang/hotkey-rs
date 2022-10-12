#[cfg(target_os = "linux")]
use std::ptr;
#[cfg(target_os = "linux")]
use x11_dl::xlib;
pub struct Utils {}
impl Utils {
    const KEY_CODE: u64 = 0x65;
    const MODIFIERS: u32 = xlib::ControlMask | xlib::ShiftMask;
    pub fn register_hotkey() {
        unsafe {
            let xlib = xlib::Xlib::open().unwrap();
            let display = (xlib.XOpenDisplay)(ptr::null());
            let root = (xlib.XDefaultRootWindow)(display);
            let keycode = (xlib.XKeysymToKeycode)(display, Self::KEY_CODE) as i32;
            println!("ready to register hotkey");
            let result = (xlib.XGrabKey)(
                display,
                keycode,
                Self::MODIFIERS,
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

    pub fn unregister_hotkey() {
        unsafe {
            let xlib = xlib::Xlib::open().unwrap();
            let display = (xlib.XOpenDisplay)(ptr::null());
            let root = (xlib.XDefaultRootWindow)(display);
            let keycode = (xlib.XKeysymToKeycode)(display, Self::KEY_CODE) as i32;
            let result = (xlib.XUngrabKey)(display, keycode, Self::MODIFIERS, root);
            if result == 0 {
                println!("UnregisterHokey succeeded");
            } else {
                println!("UnregisterHokey failed");
            }
        }
    }
}

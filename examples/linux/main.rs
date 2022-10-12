#[cfg(not(target_os = "linux"))]
fn main() {
    println!("please run this example on darwin");
}
#[cfg(target_os = "linux")]
use x11_dl::xlib;
#[cfg(target_os = "linux")]
fn main() {
    let xlib = xlib::Xlib::open().unwrap();
    unsafe {
        let display = (xlib.XOpenDisplay)(ptr::null());
        let root = (xlib.XDefaultRootWindow)(display);
        const KEY_CODE: i32 = 'A'.into();
        const MODIFIERS: i32 = xlib::ControlMask | xlib::ShiftMask;
        {
            let result = (xlib.XGrabKey)(
                display,
                KEY_CODE,
                MODIFIERS,
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
        {
            let result = (xlib.XUngrabKey)(display, KEY_CODE, MODIFIERS, root);
            if result == 0 {
                println!("UnregisterHokey succeeded");
            } else {
                println!("UnregisterHokey failed");
            }
        }
    }
    println!("Hello World!");
}

#[cfg(not(target_os = "windows"))]
fn main() {
    println!("please run this example on windows");
}
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL,
};

#[cfg(target_os = "windows")]
fn main() {
    let modifiers = MOD_CONTROL | MOD_ALT;
    unsafe {
        const ID: i32 = 0;

        {
            let result = RegisterHotKey(HWND::default(), ID, modifiers, 'A'.into());
            if result.0 == 0 {
                panic!("RegisterHotKey failed");
            } else {
                println!("RegisterHotKey succeeded");
            }
        }
        {
            let result = UnregisterHotKey(HWND::default(), ID);
            if result.0 == 0 {
                panic!("UnregisterHotKey failed");
            } else {
                println!("UnregisterHotKey succeeded");
            }
        }
    }
}

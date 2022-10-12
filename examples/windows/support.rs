use windows::Win32::Foundation::HWND;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, MOD_ALT, MOD_CONTROL,
};
pub struct Utils {}
impl Utils {
    const ID: i32 = 0;

    pub fn register_hotkey() {
        unsafe {
            let modifiers = MOD_CONTROL | MOD_ALT;
            let result = RegisterHotKey(HWND::default(), Self::ID, modifiers, 0x65);
            if result.0 == 0 {
                panic!("RegisterHotKey failed");
            } else {
                println!("RegisterHotKey succeeded");
            }
        }
    }

    pub fn unregister_hotkey() {
        unsafe {
            let result = UnregisterHotKey(HWND::default(), Self::ID);
            if result.0 == 0 {
                panic!("UnregisterHotKey failed");
            } else {
                println!("UnregisterHotKey succeeded");
            }
        }
    }
}

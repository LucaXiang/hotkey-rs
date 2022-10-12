#[cfg(not(target_os = "windows"))]
mod no_support;
#[cfg(not(target_os = "windows"))]
pub use no_support::Utils;

#[cfg(target_os = "windows")]
mod support;
#[cfg(target_os = "windows")]
pub use support::Utils;

fn main() {
    Utils::register_hotkey();
    Utils::unregister_hotkey();
}

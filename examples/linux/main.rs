#[cfg(not(target_os = "linux"))]
mod no_support;
#[cfg(not(target_os = "linux"))]
pub use no_support::Utils;

#[cfg(target_os = "linux")]
mod support;
#[cfg(target_os = "linux")]
pub use support::Utils;

fn main() {
    Utils::register_hotkey();
    Utils::unregister_hotkey();
}

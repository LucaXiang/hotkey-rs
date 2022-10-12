#[cfg(not(target_os = "macos"))]
mod no_support;
#[cfg(not(target_os = "macos"))]
pub use no_support::Utils;

#[cfg(target_os = "macos")]
mod support;
#[cfg(target_os = "macos")]
pub use support::Utils;

fn main() {
    Utils::register_hotkey();
    Utils::unregister_hotkey();
}

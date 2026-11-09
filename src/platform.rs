/// The window object returned by open_window.
pub trait Window {
    /// Tell the user if the window is still open or not.
    fn is_open(&self) -> bool;

    /// Handle all OS events.
    fn handle_events(&mut self);

    /// Sleep for timeout or until new events arrive.
    fn sleep(&self, timeout: Duration);
}

#[cfg(target_os = "linux")]
mod linux;

use std::time::Duration;

#[cfg(target_os = "linux")]
pub use linux::*;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::*;

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
compile_error!("Only MacOS and Linux are supported");

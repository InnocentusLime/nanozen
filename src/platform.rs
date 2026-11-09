pub trait Window {
    fn keep_running(&self) -> bool;

    fn handle_events(&mut self);

    fn sleep(&self, time: Duration);
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

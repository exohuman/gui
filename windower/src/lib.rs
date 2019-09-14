#![allow(unreachable_code)]
#![allow(dead_code)]
#![allow(unused_macros)]
#![allow(unused_imports)]

#[cfg(unix)]
extern crate xcb;

#[cfg(windows)]
extern crate winapi;

pub mod window;

#[cfg(unix)]
pub mod linux_window;

#[cfg(windows)]
pub mod windows_window;

use window::{Window, WindowConfig};

pub fn create_window(config: WindowConfig) -> impl Window {
    #[cfg(unix)] {
        return linux_window::LinuxWindow::create(config);
    }
    #[cfg(windows)] {
        return windows_window::WindowsWindow::create(config);
    }
    panic!("Could not detect platform")
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

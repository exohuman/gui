#[cfg(unix)]
extern crate xcb;

#[cfg(winapi)]
extern crate winapi;

pub mod window;

#[cfg(unix)]
pub mod linux_window;

use window::{Window, WindowConfig};


pub fn create_window(config: WindowConfig) -> impl Window {
    if cfg!(unix) {
        linux_window::LinuxWindow::create(config)
    }
    else if cfg!(windows) {
        panic!("Platform not supported yet");
    }
    else {
        panic!("Could not detect platform")
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

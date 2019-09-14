use crate::window::{Window, WindowConfig};

use std::ffi::{CString, CStr};
use std::{mem, ptr};
use std::convert::TryInto;

#[cfg(windows)]
use winapi;

use winapi::shared::minwindef::{LRESULT, WPARAM, LPARAM, UINT, HINSTANCE, WORD};
use winapi::shared::windef::{HWND, POINT, PSIZE, HBRUSH};

use winapi::um::libloaderapi::{GetModuleHandleExW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS};
use winapi::um::wingdi::{BLACK_BRUSH, GetStockObject};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::wincon::{AttachConsole, SetConsoleTitleW};
use winapi::um::processthreadsapi::GetCurrentProcessId;
use winapi::um::winuser::{DispatchMessageW, WNDCLASSEXW, CS_HREDRAW, CS_VREDRAW, WNDPROC, WM_PAINT, WM_SIZE, WM_DESTROY, IDC_ARROW, IDI_APPLICATION, LoadIconW, LoadCursorW, RegisterClassExW, PostQuitMessage, DefWindowProcW, DestroyWindow, ValidateRect, MAKEINTRESOURCEW, MessageBoxExW, MB_ICONERROR, GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN, CreateWindowExW, WS_OVERLAPPEDWINDOW, WS_CLIPSIBLINGS, WS_CLIPCHILDREN, WS_EX_OVERLAPPEDWINDOW, SW_SHOW, ShowWindow, SetForegroundWindow, MSG, TranslateMessage, GetMessageW};
use winapi::shared::ntdef::{MAKELANGID, LPCWSTR, LANG_NEUTRAL, SUBLANG_NEUTRAL};


fn wtext(s: &str) -> Vec<u16> {
    s.encode_utf16().collect::<Vec<u16>>()
}


#[cfg(windows)]
#[allow(dead_code)]
pub struct WindowsWindow {
    config: WindowConfig,
    h_instance: HINSTANCE,
    window_handle: HWND
}


impl WindowsWindow {
    fn create_windows_window(config: WindowConfig) -> Self {
        let title = wtext(config.title);
        let application_name = wtext(config.application_name);
        let main_name = wtext("main\0");

        let h_instance = unsafe {
            let mut h_module: HINSTANCE = ptr::null_mut();
            GetModuleHandleExW(GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS, main_name.as_ptr(), &mut h_module as *mut HINSTANCE);
            h_module
        };

        // show a console window to use to print to while we run our actual window
        unsafe {
            AllocConsole();
            AttachConsole(GetCurrentProcessId());
            SetConsoleTitleW(title.as_ptr());
        }

        let wnd_class = WNDCLASSEXW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: h_instance,
            hIcon: unsafe { LoadIconW(h_instance, MAKEINTRESOURCEW(IDI_APPLICATION as WORD)) },
            hCursor: unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) },
            hbrBackground: unsafe { GetStockObject(BLACK_BRUSH.try_into().unwrap()) as HBRUSH },
            lpszMenuName: ptr::null(),
            lpszClassName: application_name.as_ptr(),
            hIconSm: unsafe { LoadIconW(h_instance, MAKEINTRESOURCEW(IDI_APPLICATION as WORD)) },
            cbSize: mem::size_of::<WNDCLASSEXW>() as u32
        };

        let registration = unsafe {
            let wnd_class_ptr: *const WNDCLASSEXW = &wnd_class;
            if RegisterClassExW(wnd_class_ptr) <= 0 {
                exit_on_error(application_name.as_ptr(), wtext("Cannot register window").as_ptr());
            }
        };

        let (window_x, window_y) = Self::get_center(config);

        let window_handle = unsafe {
            CreateWindowExW(
                WS_EX_OVERLAPPEDWINDOW,
                application_name.as_ptr(),
                application_name.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_CLIPSIBLINGS | WS_CLIPCHILDREN,
                window_x,
                window_y,
                config.width,
                config.height,
                ptr::null_mut(),
                ptr::null_mut(),
                h_instance,
                ptr::null_mut()
            )
        };

        if window_handle.is_null() {
            let msg = wtext("Failed to create window\0");
            unsafe { exit_on_error(application_name.as_ptr(), msg.as_ptr()) };
        }

        WindowsWindow {
            config,
            h_instance,
            window_handle,
        }
    }

    fn show_windows_window(&self) -> String {
        unsafe {
            ShowWindow(self.window_handle, SW_SHOW);
            SetForegroundWindow(self.window_handle);
            SetForegroundWindow(self.window_handle);
        };
        String::new()
    }

    fn render_loop_windows(&self) {
        unsafe {
            let mut message: MSG = mem::zeroed();
            loop {
                if GetMessageW(&mut message as *mut MSG, self.window_handle, 0, 0) > 0 {
                    TranslateMessage(&message as *const MSG);
                    DispatchMessageW(&message as *const MSG);
                }
                else {
                    break;
                }
            }
        }
    }

    fn get_center(config: WindowConfig) -> (i32, i32) {
        let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
        let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

        let window_left = screen_width / 2 - config.width / 2;
        let window_top = screen_height / 2 - config.height / 2;
        (window_left, window_top)
    }
}


impl Window for WindowsWindow {
    fn create(config: WindowConfig) -> Self {
        WindowsWindow::create_windows_window(config)
    }

    fn show(&self) -> String {
        self.show_windows_window();
        String::from("done")
    }

    fn render_loop(&self) {
        self.render_loop_windows();
    }
}


pub unsafe extern "system" fn window_proc(h_wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    println!("start window_proc");
    match message {
        WM_DESTROY => {
            println!("WM_DESTROY");
            DestroyWindow(h_wnd);
            PostQuitMessage(0);
        },
        WM_PAINT => {
            ValidateRect(h_wnd, ptr::null());
            println!("WM_PAINT");
        },
        WM_SIZE => {
            println!("WM_SIZE");
        },
        _ => {
        }
    };
    let result = DefWindowProcW(h_wnd, message, w_param, l_param);
    println!("end window_proc");
    result
}


pub unsafe extern "system" fn exit_on_error(caption: LPCWSTR, msg: LPCWSTR) {
    MessageBoxExW(ptr::null_mut(), msg, caption, MB_ICONERROR, MAKELANGID(LANG_NEUTRAL, SUBLANG_NEUTRAL));
}

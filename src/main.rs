// https://stackoverflow.com/questions/29763647/how-to-make-a-program-that-does-not-display-the-console-window
// #![windows_subsystem = "windows"]

fn main() {
    register_window_class();
    create_window();
    message_loop();
}

// msgbox("你好").unwrap();

#[allow(dead_code)]
fn msgbox(msg: &str) -> Result<i32, std::io::Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;

    use winapi::um::winuser::{MB_OK, MessageBoxW};

    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();

    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };

    if ret == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(ret)
    }
}

#[allow(dead_code)]
fn get_hinstance() {
    use std::ptr::null;
    use winapi::shared::minwindef::HINSTANCE;
    use winapi::um::libloaderapi::GetModuleHandleW;

    let handle: HINSTANCE = unsafe { GetModuleHandleW(null()) };

    println!("hinstance = {:?}", handle);
}

fn register_window_class() {
    use std::ptr::{null, null_mut};
    use winapi::shared::minwindef::HINSTANCE;
    use winapi::shared::minwindef::ATOM;
    use winapi::shared::windef::HBRUSH;
    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::winuser::{WNDCLASSW, CS_HREDRAW, CS_VREDRAW};
    use winapi::um::winuser::{LoadIconW, IDI_APPLICATION};
    use winapi::um::winuser::{LoadCursorW, IDC_ARROW};
    use winapi::um::winuser::COLOR_BTNFACE;
    use winapi::um::winuser::RegisterClassW;
    use winapi::um::errhandlingapi::GetLastError;

    let hinstance: HINSTANCE = unsafe { GetModuleHandleW(null()) };

    let class_name = utf8_to_utf16("RustWin");

    let wc = WNDCLASSW{
        style: CS_HREDRAW | CS_VREDRAW,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: hinstance,
        hIcon: unsafe { LoadIconW(null_mut(), IDI_APPLICATION) },
        hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) },
        hbrBackground: (COLOR_BTNFACE + 1) as HBRUSH,
        lpszMenuName: null(),
        lpszClassName: class_name.as_ptr(),
    };

    let atom: ATOM = unsafe { RegisterClassW(&wc as *const WNDCLASSW) };
    if atom == 0 {
        panic!("RegisterClassW: {}", unsafe { GetLastError() });
    }

    println!("atom: {}", atom);
}

fn create_window() {
    use std::ptr::{null, null_mut};

    use winapi::um::winuser::WS_OVERLAPPEDWINDOW;
    use winapi::um::winuser::CW_USEDEFAULT;
    use winapi::um::winuser::SW_SHOWNORMAL;

    use winapi::um::libloaderapi::GetModuleHandleW;
    use winapi::um::winuser::{CreateWindowExW, ShowWindow, UpdateWindow};


    let class_name = utf8_to_utf16("RustWin");
    let window_name = utf8_to_utf16("Hello Rust");

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            window_name.as_ptr(),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            null_mut(),
            null_mut(),
            GetModuleHandleW(null()),
            null_mut())
    };

    unsafe { ShowWindow(hwnd, SW_SHOWNORMAL) };
    unsafe { UpdateWindow(hwnd) };
}

fn message_loop() {
    use std::ptr::null_mut;

    use winapi::shared::minwindef::TRUE;
    use winapi::um::winuser::MSG;
    use winapi::um::winuser::{GetMessageW, TranslateMessage, DispatchMessageW};

    unsafe {
        let mut msg: MSG = std::mem::zeroed();

        while GetMessageW(&mut msg, null_mut(), 0, 0) == TRUE {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

use winapi::shared::minwindef::{LPARAM, LRESULT, UINT, WPARAM};
use winapi::shared::windef::HWND;

unsafe extern "system" fn window_proc(hwnd: HWND, message: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    use std::ptr::null_mut;
    use winapi::shared::minwindef::{DWORD, HIWORD};
    use winapi::um::winuser::{WM_COMMAND, WM_CREATE, WM_CLOSE};
    use winapi::um::winuser::{WS_CHILD, WS_VISIBLE, BS_PUSHBUTTON};
    use winapi::um::winuser::BN_CLICKED;
    use winapi::um::winuser::CREATESTRUCTW;
    use winapi::um::winuser::CreateWindowExW;
    use winapi::um::winuser::{DefWindowProcW, PostQuitMessage};
    use winapi::um::winuser::{MB_OK, MessageBoxW};

    match message {
        WM_CREATE => {
            let button_hwnd = CreateWindowExW(
                0,
                utf8_to_utf16("BUTTON").as_ptr(),
                utf8_to_utf16("你好").as_ptr(),
                WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
                10, 10,
                100, 30,
                hwnd,
                null_mut(),
                (*(lparam as *const CREATESTRUCTW)).hInstance,
                null_mut());

            println!("button: {:?}", button_hwnd);

            0
        },
        WM_COMMAND => {
            if HIWORD(wparam as DWORD) == BN_CLICKED {
                // FIXME the empty string
                MessageBoxW(
                    hwnd,
                    utf8_to_utf16("This is AMAZING!").as_ptr(),
                    utf8_to_utf16("").as_ptr(),
                    MB_OK);
            }
            0
        },
        WM_CLOSE => {
            PostQuitMessage(0);
            0
        },
        _ => DefWindowProcW(hwnd, message, wparam, lparam)
    }
}

fn utf8_to_utf16<S: AsRef<std::ffi::OsStr> + ?Sized>(s: &S) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::iter::once; // '\0'
    use std::os::windows::ffi::OsStrExt; // encode_wide()

    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

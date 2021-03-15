#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

extern crate sciter;

use std::ptr::null_mut;
use winapi::shared::{minwindef, windef};
use winapi::um::{libloaderapi, winuser};
mod windowAlingment;
use windowAlingment::*;

static mut dupa:bool = true;

pub fn main() {
    let windowAlingment = WINDOWPOS::new(
        get_desktop_resolution(),
        45,
        15_f32,
        43_f32,
        WINDOWALINGMENT::BottomLeft,
    );

    let windowHwnd = unsafe { create_window(window_proc, windowAlingment) };
    unsafe { winuser::AddClipboardFormatListener(windowHwnd) };
    let frame = sciter::Window::attach(windowHwnd as sciter::types::HWINDOW);

    unsafe {
        winuser::ShowWindow(windowHwnd, winuser::SW_SHOW);
        let mut msg: winuser::MSG = std::mem::zeroed();
        while winuser::GetMessageA(&mut msg, null_mut(), 0, 0) != 0 {
            winuser::TranslateMessage(&msg);
            winuser::DispatchMessageA(&msg);
        }
        let monitorInfo: winuser::MONITORENUMPROC = std::mem::zeroed();
        winuser::EnumDisplayMonitors(null_mut(), null_mut(), monitorInfo, 0);
    }
}

fn get_desktop_resolution() -> (i32, i32) {
    let mut desktop_rect: windef::RECT = unsafe { std::mem::zeroed() };
    unsafe { winuser::GetWindowRect(winuser::GetDesktopWindow(), &mut desktop_rect) };
    // width, height
    (desktop_rect.right, desktop_rect.bottom)
}

unsafe fn create_window(
    window_procedure: unsafe extern "system" fn(
        windef::HWND,
        u32,
        minwindef::WPARAM,
        minwindef::LPARAM,
    ) -> minwindef::LRESULT,
    aling_pos: WINDOWPOS,
) -> windef::HWND {
    let class_name: &[u8] = b"rust_clipboard_manager\0";
    let window_name: &[u8] = b"Clipboard Manager\0";
    // masz bojowe zadanie, wymyslic ładne nazwy klas i okna. hehe
    let mut window_class: winuser::WNDCLASSA = std::mem::zeroed();
    window_class.hInstance = libloaderapi::GetModuleHandleA(null_mut());
    window_class.lpfnWndProc = Some(window_procedure);
    window_class.lpszClassName = class_name.as_ptr() as *const i8;

    if winuser::RegisterClassA(&window_class) == 0 {
        winuser::MessageBoxA(
            null_mut(),
            "Failed to register window!\0".as_ptr() as *mut i8,
            "Error\0".as_ptr() as *mut i8,
            winuser::MB_APPLMODAL | winuser::MB_OK,
        );
        return null_mut();
    }

    let (window_posX, window_posY) = aling_pos.getWindowPos();
    let (window_width, window_height) = aling_pos.getSize();

    let hwnd = winuser::CreateWindowExA(
        winuser::WS_EX_TOPMOST,
        class_name.as_ptr() as *const i8,
        window_name.as_ptr() as *const i8,
        //winuser::WS_EX_LAYERED | winuser::WS_EX_TRANSPARENT | winuser::WS_EX_TOPMOST,
        //winuser::WS_OVERLAPPED | winuser::WS_VISIBLE,
        //winuser::WS_VISIBLE | winuser::WS_POPUP,
        winuser::WS_POPUP | winuser::WS_VISIBLE,
        window_posX, // x
        window_posY, // y
        window_width,
        window_height,
        null_mut(),
        null_mut(),
        libloaderapi::GetModuleHandleA(null_mut()),
        null_mut(),
    );
    if hwnd != null_mut() {
        return hwnd;
    } else {
        return null_mut();
    }
}

pub unsafe extern "system" fn window_proc(
    hwnd: windef::HWND,
    uMsg: u32,
    wParam: minwindef::WPARAM,
    lParam: minwindef::LPARAM,
) -> minwindef::LRESULT {
    let sciterApiRef = sciter::SciterAPI();
    //struct holding all adresses of sciterApi function pointers
    type FncMessageHandlePtr =
        extern "system" fn(sciter::types::HWINDOW, u32, usize, isize, *mut i32) -> isize; //alias for function pointer type
    type FncLoadFilePtr =
        extern "system" fn(sciter::types::HWINDOW, sciter::types::LPCWSTR) -> minwindef::BOOL;

    let sciterFncMessageHandle: FncMessageHandlePtr = sciterApiRef.SciterProcND;
    let sciterFncLoadFile: FncLoadFilePtr = sciterApiRef.SciterLoadFile;

    let mut handledBySciter: minwindef::BOOL = 0;
    let lResult = sciterFncMessageHandle(
        hwnd as sciter::types::HWINDOW,
        uMsg,
        wParam,
        lParam,
        &mut handledBySciter,
    );
    if handledBySciter != 0 {
        return lResult;
    }
    match uMsg {
        winuser::WM_CREATE => {
            let name: Vec<u16> =
                "F:\\Projekty\\RUST\\GUI\\Sciter\\ClipboardManager\\src\\index.htm"
                    .encode_utf16()
                    .collect();
            sciterFncLoadFile(hwnd as sciter::types::HWINDOW, name.as_ptr());
        }
        winuser::WM_CLOSE => {
            winuser::DestroyWindow(hwnd);
        }
        winuser::WM_DESTROY => {
            winuser::PostQuitMessage(69);
        }
        winuser::WM_CLIPBOARDUPDATE => {
        if dupa == true{
            winuser::ShowWindow(hwnd, winuser::SW_HIDE);
            dupa = false;
        }   else if dupa == false
        {
            winuser::ShowWindow(hwnd, winuser::SW_SHOW);
            dupa = true;
        }
        }
        winuser::WM_DISPLAYCHANGE => {
            let (width, height) = get_desktop_resolution();
            let arg: String = format!("Width: {} Height: {}\0", width, height);
            winuser::MessageBoxA(
                null_mut(),
                arg.as_ptr() as *mut i8,
                "okienko\0".as_ptr() as *const i8,
                winuser::MB_APPLMODAL | winuser::MB_OK,
            );
        }
        _ => {}
    }
    return winuser::DefWindowProcA(hwnd, uMsg, wParam, lParam);
}

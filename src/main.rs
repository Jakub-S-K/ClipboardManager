#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

extern crate sciter;

use std::ptr::null_mut;
use winapi::shared::{minwindef, windef};
use winapi::um::{winuser, libloaderapi, wingdi};

pub unsafe extern "system" fn window_proc(hwnd: windef::HWND, uMsg: u32, wParam: minwindef::WPARAM, lParam: minwindef::LPARAM) -> minwindef::LRESULT
{
    let sciterApiRef = sciter::SciterAPI();
    //struct holding all adresses of sciterApi function pointers
    type FncMessageHandlePtr = extern "system" fn(sciter::types::HWINDOW, u32, usize, isize, *mut i32) -> isize; //alias for function pointer type
    type FncLoadFilePtr = extern "system" fn (sciter::types::HWINDOW, sciter::types::LPCWSTR) -> minwindef::BOOL;

    let sciterFncMessageHandle: FncMessageHandlePtr = sciterApiRef.SciterProcND;
    let sciterFncLoadFile: FncLoadFilePtr = sciterApiRef.SciterLoadFile;
    
    let mut handledBySciter: minwindef::BOOL = 0;
    let lResult = sciterFncMessageHandle(hwnd as sciter::types::HWINDOW, uMsg, wParam, lParam, &mut handledBySciter);
    
    if handledBySciter != 0 {
        return lResult;
    }

    match uMsg
    {
        winuser::WM_CREATE => 
        {
            let name: Vec<u16> = "F:\\Projekty\\RUST\\GUI\\Sciter\\ClipboardManager\\src\\index.htm".encode_utf16().collect();
            sciterFncLoadFile(hwnd as sciter::types::HWINDOW, name.as_ptr());
        }
        winuser::WM_CLOSE =>
        {
            winuser::DestroyWindow(hwnd);
        }
        winuser::WM_DESTROY =>
        {
            winuser::PostQuitMessage(69);
        }
        _ => {}
    }
    return winuser::DefWindowProcA(hwnd, uMsg, wParam, lParam)
}

pub fn main() 
{
    let class_name: &[u8] = b"hohoKlasaGoesBrrr\0";
    let window_name: &[u8] = b"dupa\0"; 
    unsafe {
        let mainHinstance: minwindef::HINSTANCE = libloaderapi::GetModuleHandleA(null_mut());
        let mut wc: winuser::WNDCLASSA = std::mem::zeroed();
        wc.hInstance = mainHinstance; 
        wc.lpfnWndProc = Some(window_proc);
        wc.lpszClassName = class_name.as_ptr() as *const i8;
        if winuser::RegisterClassA(&wc) == 0
        {
            winuser::MessageBoxA(null_mut(), "Failed to register window!".as_ptr() as *mut i8, "Error".as_ptr() as *mut i8, winuser::MB_APPLMODAL | winuser::MB_OK);
            return;
        }
        let windowHwnd = winuser::CreateWindowExA(
            winuser::WS_EX_LAYERED | winuser::WS_EX_TRANSPARENT | winuser::WS_EX_TOPMOST,
            class_name.as_ptr() as *const i8,
            window_name.as_ptr() as *const i8,
            //winuser::WS_OVERLAPPED | winuser::WS_VISIBLE,
            winuser::WS_VISIBLE | winuser::WS_POPUP,
            winuser::CW_USEDEFAULT, 
            winuser::CW_USEDEFAULT, 
            4560, 
            900, 
            null_mut(),
            null_mut(),
            mainHinstance,
            null_mut(),
        );
        if windowHwnd == null_mut()
        {
            panic!("DUPA");
        }
        
        let frame = sciter::Window::attach(windowHwnd as sciter::types::HWINDOW);
        winuser::SetLayeredWindowAttributes(windowHwnd, wingdi::RGB(255, 255, 255), 200, winuser::LWA_ALPHA | winuser::LWA_COLORKEY);

        winuser::ShowWindow(windowHwnd, winuser::SW_RESTORE);
        let mut msg: winuser::MSG = std::mem::zeroed();

        while winuser::GetMessageA(&mut msg, null_mut(), 0, 0) > 0 {
            winuser::TranslateMessage(&msg);
            winuser::DispatchMessageA(&msg);
        }
    }
}

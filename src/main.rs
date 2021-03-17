#![windows_subsystem = "windows"]
extern crate memory_module_sys;
extern crate sciter;

use std::ptr::null_mut;
use winapi::shared::{minwindef, windef};
use winapi::um::{libloaderapi, winuser};
#[allow(non_snake_case)]
mod windowAlingment;
use windowAlingment::*;

#[allow(non_snake_case)]
pub fn main() {
    let windowAlingment = WINDOWPOS::new(
        getDesktopResolution(),
        45,
        15_f32,
        43_f32,
        WINDOWALINGMENT::BottomLeft,
    );
    let windowHwnd = unsafe { createWindow(windowProc, windowAlingment) };
    unsafe { winuser::AddClipboardFormatListener(windowHwnd) };
    let frame = sciter::Window::attach(windowHwnd as sciter::types::HWINDOW);

    //frame.load_html(binHtml, Some("this://main.htm"));
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

#[allow(non_snake_case)]
fn getDesktopResolution() -> (i32, i32) {
    let mut desktopRect: windef::RECT = unsafe { std::mem::zeroed() };
    unsafe { winuser::GetWindowRect(winuser::GetDesktopWindow(), &mut desktopRect) };
    // width, height
    (desktopRect.right, desktopRect.bottom)
}

#[allow(non_snake_case)]
unsafe fn createWindow(
    windowProcedure: unsafe extern "system" fn(
        windef::HWND,
        u32,
        minwindef::WPARAM,
        minwindef::LPARAM,
    ) -> minwindef::LRESULT,
    alignPos: WINDOWPOS,
) -> windef::HWND {
    let className: &[u8] = b"rust_clipboard_manager\0";
    let windowName: &[u8] = b"Clipboard Manager\0";
    ourMessageBoxU8(className);
    ourMessageBoxU8(windowName);
    // masz bojowe zadanie, wymyslic ładne nazwy klas i okna. hehe
    let mut windowClass: winuser::WNDCLASSA = std::mem::zeroed();
    windowClass.hInstance = libloaderapi::GetModuleHandleA(null_mut());
    windowClass.lpfnWndProc = Some(windowProcedure);
    windowClass.lpszClassName = className.as_ptr() as *const i8;

    if winuser::RegisterClassA(&windowClass) == 0 {
        winuser::MessageBoxA(
            null_mut(),
            "Failed to register window!\0".as_ptr() as *mut i8,
            "Error\0".as_ptr() as *mut i8,
            winuser::MB_APPLMODAL | winuser::MB_OK,
        );
        return null_mut();
    }

    let (windowPosX, windowPosY) = alignPos.getWindowPos();
    let (windowWidth, windowHeight) = alignPos.getSize();
    let hwnd = winuser::CreateWindowExA(
        winuser::WS_EX_TOPMOST,
        className.as_ptr() as *const i8,
        windowName.as_ptr() as *const i8,
        //winuser::WS_EX_LAYERED | winuser::WS_EX_TRANSPARENT | winuser::WS_EX_TOPMOST,
        //winuser::WS_OVERLAPPED | winuser::WS_VISIBLE,
        //winuser::WS_VISIBLE | winuser::WS_POPUP,
        winuser::WS_POPUP | winuser::WS_VISIBLE,
        windowPosX, // x
        windowPosY, // y
        windowWidth,
        windowHeight,
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

#[allow(non_snake_case)]
pub unsafe extern "system" fn windowProc(
    hwnd: windef::HWND,
    uMsg: u32,
    wParam: minwindef::WPARAM,
    lParam: minwindef::LPARAM,
) -> minwindef::LRESULT {
    let sciterApiRef = sciter::SciterAPI();
    //struct holding all adresses of sciterApi function pointers
    type FncMessageHandlePtr =
        extern "system" fn(sciter::types::HWINDOW, u32, usize, isize, *mut i32) -> isize; //alias for function pointer type
    type FncLoadFilePtr = extern "system" fn(
        sciter::types::HWINDOW,
        sciter::types::LPCBYTE,
        u32,
        sciter::types::LPCWSTR,
    ) -> minwindef::BOOL;
    let sciterFncMessageHandle: FncMessageHandlePtr = sciterApiRef.SciterProcND;
    let sciterFncLoadHtml: FncLoadFilePtr = sciterApiRef.SciterLoadHtml;
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
            match std::env::current_dir() {
                Ok(dupa) => match dupa.to_str() {
                    Some(text) => {
                        ourMessageBoxS(text);
                    }
                    None => {
                        panic!("Couldn't yield path");
                    }
                },
                Err(tak) => {
                    panic!("Couldn't yield path with error: {}", tak);
                }
            }
            //(sciterApiRef.SciterSetCallback)(hwnd as sciter::types::HWINDOW, HostCallbackFnc, null_mut());
            //let binGif = include_bytes!("src\\frontend\\data\\someRealShit.gif");
            //let htmlInternalPath: Vec<u16> = String::from("this://someRealShit.gif\0").encode_utf16().collect();
            //let htmlInternalPath: Vec<u16> = "F:\\Projekty\\RUST\\GUI\\Sciter\\ClipboardManager\\src\\frontend\\data\\someRealShit.gif".encode_utf16().collect();
            //(sciterApiRef.SciterLoadFile)(hwnd as sciter::types::HWINDOW, htmlInternalPath.as_ptr());

            //let binHtml = include_bytes!("F:\\Projekty\\RUST\\GUI\\Sciter\\ClipboardManager\\src\\frontend\\index.htm");
            //let htmlInternalPath: Vec<u16> = String::from("this://index.htm\0").encode_utf16().collect();
            //(sciterApiRef.SciterLoadHtml)(hwnd as sciter::types::HWINDOW, binHtml.as_ptr(), std::mem::size_of_val(binHtml) as u32, htmlInternalPath.as_ptr());
        }
        winuser::WM_CLOSE => {
            winuser::DestroyWindow(hwnd);
        }
        winuser::WM_DESTROY => {
            winuser::PostQuitMessage(69);
        }
        winuser::WM_CLIPBOARDUPDATE => {
            winuser::ShowWindow(hwnd, winuser::SW_SHOW);
        }
        winuser::WM_DISPLAYCHANGE => {
            let (width, height) = getDesktopResolution();
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

#[allow(non_snake_case)]
unsafe fn ourMessageBox(textToDisplay: String) {
    winuser::MessageBoxA(
        null_mut(),
        textToDisplay.as_ptr() as *const i8,
        "dupa\0".as_ptr() as *const i8,
        winuser::MB_APPLMODAL | winuser::MB_OK,
    );
}


#[allow(non_snake_case)]
unsafe fn ourMessageBoxS(textToDisplay: &str) {
    let mut textToDisplay = String::from(textToDisplay);
    textToDisplay += "\0";
    winuser::MessageBoxA(
        null_mut(),
        textToDisplay.as_ptr() as *const i8,
        "dupa\0".as_ptr() as *const i8,
        winuser::MB_APPLMODAL | winuser::MB_OK,
    );
}


#[allow(non_snake_case)]
unsafe fn ourMessageBoxU8(textToDisplay: &[u8]) {
    winuser::MessageBoxA(
        null_mut(),
        textToDisplay.as_ptr() as *const i8,
        "dupa\0".as_ptr() as *const i8,
        winuser::MB_APPLMODAL | winuser::MB_OK,
    );
}

/*extern "system" fn HostCallbackFnc(scn: sciter::types::LPSCITER_CALLBACK_NOTIFICATION, callbackParam: sciter::types::LPVOID) -> u32
{
    match std::mem::transmute::<u32, sciter::types::SCITER_NOTIFICATION>((*scn).code) //cast from u32 to enum
    {
        sciter::types::SCITER_NOTIFICATION::SC_LOAD_DATA =>
        {
        }
        sciter::types::SCITER_NOTIFICATION::SC_DATA_LOADED =>
        {

        }
        sciter::types::SC
        _ => todo!();
    }
    unimplemented!
}*/

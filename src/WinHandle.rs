use std::ptr::null_mut;
use winapi::{
    shared::{minwindef, windef},
    um::{libloaderapi, wingdi, winuser},
};

extern crate sciter;

pub struct WinHandle {
    hwnd: windef::HWND,
}

impl WinHandle {
    pub unsafe fn new(className: &[u8], windowName: &[u8], (x, y): (i32, i32)) -> Self {
        let mut windowClass: winuser::WNDCLASSA;
        windowClass = std::mem::zeroed();
        windowClass.hInstance = libloaderapi::GetModuleHandleA(null_mut());

        windowClass.lpfnWndProc = Some(WinHandle::windowProcedure);
        windowClass.lpszClassName = className.as_ptr() as *const _;
        if winuser::RegisterClassA(&windowClass) == 0 {
            WinHandle::messageBox(String::from("Failed to register class"));
            panic!();
        }
        let (width, height) = WinHandle::getDesktopResolution();
        let tempHWND: windef::HWND;

        tempHWND = winuser::CreateWindowExA(
            winuser::WS_EX_LAYERED | winuser::WS_EX_TOPMOST,
            className.as_ptr() as *const _,
            windowName.as_ptr() as *const _,
            winuser::WS_POPUP | winuser::WS_VISIBLE,
            x,
            y,
            width,
            height,
            null_mut(),
            null_mut(),
            libloaderapi::GetModuleHandleA(null_mut()),
            null_mut(),
        );
        if tempHWND == null_mut() {
            WinHandle::messageBox(String::from("Failed to create window"));
        }
        winuser::SetLayeredWindowAttributes(
            tempHWND,
            wingdi::RGB(255_u8, 255_u8, 255_u8),
            255,
            winuser::LWA_ALPHA | winuser::LWA_COLORKEY,
        );
        return Self { hwnd: tempHWND };
    }
    fn getHWND(&self) -> windef::HWND {
        self.hwnd
    }
    fn getDesktopResolution() -> (i32, i32) {
        let mut desktopRect: windef::RECT = unsafe { std::mem::zeroed() };
        unsafe { winuser::GetWindowRect(winuser::GetDesktopWindow(), &mut desktopRect) };
        // width, height
        (desktopRect.right, desktopRect.bottom)
    }
    pub unsafe fn messageBox(textToDisplay: String) {
        let textToDisplay = textToDisplay + "\0";
        winuser::MessageBoxA(
            null_mut(),
            textToDisplay.as_ptr() as *const _,
            "WinHandle MessageBox".as_ptr() as *const _,
            winuser::MB_APPLMODAL | winuser::MB_OK,
        );
    }
    unsafe extern "system" fn windowProcedure(
        hwnd: windef::HWND,
        uMsg: u32,
        wParam: minwindef::WPARAM,
        lParam: minwindef::LPARAM,
    ) -> minwindef::LRESULT {
        let sciterApiRef = sciter::SciterAPI();
        //struct holding all adresses of sciterApi function pointers

        let mut handledBySciter: minwindef::BOOL = 0;
        let lResult = (sciterApiRef.SciterProcND)(
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
                //ourMessageBoxS(std::env::current_dir().expect("Couldn't yield path").to_str().unwrap());
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
                let (width, height) = WinHandle::getDesktopResolution();
                let arg: String = format!("Width: {} Height: {}", width, height);
                WinHandle::messageBox(arg);
            }
            _ => {}
        }
        winuser::DefWindowProcA(hwnd, uMsg, wParam, lParam)
    }
    unsafe fn hookClipboardListener(&self) {
        winuser::AddClipboardFormatListener(self.hwnd);
    }
    unsafe fn messageLoop(&self) {
        winuser::ShowWindow(self.hwnd, winuser::SW_SHOW);
        let mut msg: winuser::MSG = std::mem::zeroed();
        while winuser::GetMessageA(&mut msg, null_mut(), 0, 0) != 0 {
            winuser::TranslateMessage(&msg);
            winuser::DispatchMessageA(&msg);
        }
    }
}

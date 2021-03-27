use std::ptr::null_mut;
use winapi::{
    shared::{minwindef, windef},
    um::{libloaderapi, wingdi, winuser},
};

use crate::Clipboard::ClipboardHandler;

extern crate sciter;

pub struct WinHandler {
    hwnd: windef::HWND,
}

impl WinHandler {
    pub fn new(className: &[u8], windowName: &[u8], windowPos: WindowPos) -> Self {
        let mut windowClass: winuser::WNDCLASSA;
        windowClass = unsafe { std::mem::zeroed() };
        windowClass.hInstance = unsafe { libloaderapi::GetModuleHandleA(null_mut()) };

        windowClass.lpfnWndProc = Some(WinHandler::windowProcedure);
        windowClass.lpszClassName = className.as_ptr() as *const _;
        windowClass.cbWndExtra = std::mem::size_of::<ClipboardHandler>() as i32;
        if unsafe { winuser::RegisterClassA(&windowClass) } == 0 {
            WinHandler::messageBox(String::from("Failed to register class"));
            panic!();
        }
        let (width, height) = windowPos.getSize();
        let tempHWND: windef::HWND;
        let (x, y) = windowPos.getWindowPos();
        tempHWND = unsafe {
            winuser::CreateWindowExA(
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
            )
        };
        if tempHWND == null_mut() {
            WinHandler::messageBox(String::from("Failed to create window"));
        }

        unsafe {
            winuser::SetLayeredWindowAttributes(
                tempHWND,
                wingdi::RGB(255_u8, 255_u8, 255_u8),
                255,
                winuser::LWA_ALPHA | winuser::LWA_COLORKEY,
            )
        };
        return WinHandler { hwnd: tempHWND };
    }
    pub fn setClipboard(&self, clipboard: *mut ClipboardHandler) {
        unsafe {
            winuser::SetWindowLongPtrA(
                self.getHWND(),
                winuser::GWL_USERDATA,
                clipboard as *mut ClipboardHandler as *mut isize as isize,
            )
        };
    }
    pub fn getHWND(&self) -> windef::HWND {
        self.hwnd
    }
    /// returns first width then height
    pub fn getDesktopResolution() -> (i32, i32) {
        let mut desktopRect: windef::RECT = unsafe { std::mem::zeroed() };
        unsafe { winuser::GetWindowRect(winuser::GetDesktopWindow(), &mut desktopRect) };
        (desktopRect.right, desktopRect.bottom)
    }
    pub fn sendMessage(&self, message: u32) {
        unsafe { winuser::SendMessageA(self.hwnd, message, 0, 0) };
    }
    pub fn messageBox(textToDisplay: String) {
        let textToDisplay = textToDisplay + "\0";
        unsafe {
            winuser::MessageBoxA(
                null_mut(),
                textToDisplay.as_ptr() as *const _,
                "WinHandle MessageBox".as_ptr() as *const _,
                winuser::MB_APPLMODAL | winuser::MB_OK,
            )
        };
    }
    unsafe extern "system" fn windowProcedure(
        hwnd: windef::HWND,
        uMsg: u32,
        wParam: minwindef::WPARAM,
        lParam: minwindef::LPARAM,
    ) -> minwindef::LRESULT {
        let sciterApiRef = sciter::SciterAPI();
        //struct holding all adresses of sciterApi function pointers
        let CLIPBOARD_INSTANCE: &mut ClipboardHandler;
        CLIPBOARD_INSTANCE = &mut *(winuser::GetWindowLongPtrA(hwnd, winuser::GWL_USERDATA)
            as *mut isize as *mut ClipboardHandler);

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
            winuser::WM_CREATE => {}
            winuser::WM_CLOSE => {
                winuser::DestroyWindow(hwnd);
            }
            winuser::WM_DESTROY => {
                winuser::PostQuitMessage(69);
            }
            winuser::WM_CLIPBOARDUPDATE => {
                //WinHandler::getClipboard();
                //handler.update();
                CLIPBOARD_INSTANCE.update();
            }
            winuser::WM_DISPLAYCHANGE => {
                let (width, height) = WinHandler::getDesktopResolution();
                let arg: String = format!("Width: {} Height: {}", width, height);
                WinHandler::messageBox(arg);
            }
            _ => {}
        }
        winuser::DefWindowProcA(hwnd, uMsg, wParam, lParam)
    }
    pub fn hookClipboardListener(&self) {
        unsafe { winuser::AddClipboardFormatListener(self.hwnd) };
    }
    pub fn messageLoop(&self) {
        unsafe {
            winuser::ShowWindow(self.hwnd, winuser::SW_SHOW);
            let mut msg: winuser::MSG = std::mem::zeroed();
            while winuser::GetMessageA(&mut msg, null_mut(), 0, 0) != 0 {
                winuser::TranslateMessage(&msg);
                winuser::DispatchMessageA(&msg);
            }
        }
    }
}

pub enum WINDOWALINGMENT {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

pub struct WindowPos {
    screen_width: i32,
    screen_height: i32,
    offsetFromBorders: i32,
    percentileOffsetHeight: f32,
    percentileOffsetWidth: f32,
    alignment: WINDOWALINGMENT,
}

impl WindowPos {
    pub fn new(
        borderOffset: i32,
        offsetWidth: f32,
        offsetHeight: f32,
        align: WINDOWALINGMENT,
    ) -> Self {
        // to fix, delete creation of additional variable
        let (x, y) = WinHandler::getDesktopResolution();
        return WindowPos {
            screen_width: x,
            screen_height: y,
            alignment: align,
            offsetFromBorders: borderOffset,
            percentileOffsetHeight: offsetHeight,
            percentileOffsetWidth: offsetWidth,
        };
    }
    /// return offset X then offset Y
    fn getWindowPos(&self) -> (i32, i32) {
        match self.alignment {
            WINDOWALINGMENT::BottomRight => {
                return (
                    self.screen_width
                        - self.percentFromVal(self.screen_width as f32, self.percentileOffsetWidth)
                            as i32
                        - self.offsetFromBorders,
                    self.screen_height
                        - self
                            .percentFromVal(self.screen_height as f32, self.percentileOffsetHeight)
                            as i32
                        - self.offsetFromBorders,
                )
            }
            WINDOWALINGMENT::BottomLeft => {
                return (
                    self.offsetFromBorders,
                    self.screen_height
                        - self
                            .percentFromVal(self.screen_height as f32, self.percentileOffsetHeight)
                            as i32
                        - self.offsetFromBorders,
                )
            }
            WINDOWALINGMENT::TopLeft => return (self.offsetFromBorders, self.offsetFromBorders),
            WINDOWALINGMENT::TopRight => {
                return (
                    self.screen_width
                        - self.percentFromVal(self.screen_width as f32, self.percentileOffsetWidth)
                            as i32
                        - self.offsetFromBorders,
                    self.offsetFromBorders,
                )
            }
        }
    }
    fn getSize(&self) -> (i32, i32) {
        return (
            self.percentFromVal(self.screen_width as f32, self.percentileOffsetWidth) as i32,
            self.percentFromVal(self.screen_height as f32, self.percentileOffsetHeight) as i32,
        );
    }
    fn percentFromVal(&self, val: f32, percent: f32) -> f32 {
        val * (percent * 0.01_f32)
    }
}

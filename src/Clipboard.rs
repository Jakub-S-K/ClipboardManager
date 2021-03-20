#[allow(non_snake_case)]
use std::ptr::null_mut;
use winapi::{
    shared::{minwindef, windef},
    um::winuser,
};

//enum ClipboardVariable {
//temp1 : std::ffi::CString, //ascii, html
//temp2 : String, //asci, utf-16
//}

pub struct ClipbaordHandler {
    clipboardType: u32,
    hwnd: windef::HWND,
}

impl ClipbaordHandler {
    fn new(tempHWND: windef::HWND) -> Self {
        ClipbaordHandler {
            clipboardType: 2,
            hwnd: tempHWND,
        }
    }
    pub fn update(&mut self) {
        unsafe { winuser::OpenClipboard(self.hwnd) };
        let amountOfFormats = unsafe { winuser::CountClipboardFormats() };
        let mut currentFormat = 0;
        for i in 0..amountOfFormats {
            currentFormat = unsafe { winuser::EnumClipboardFormats(currentFormat) };
            self.parseData(currentFormat);
        }

        unsafe { winuser::EmptyClipboard() };
        unsafe { winuser::CloseClipboard() };
    }
    fn parseData(&mut self, format: u32) {
        use winapi::um::winbase;
        use winapi::um::wingdi;
        use winapi::um::winnt;
        use winapi::um::winnt::HANDLE;
        use winuser::*;
        let mut globalPointers: Vec<HANDLE> = Vec::new();
        match format {
            CF_BITMAP => {
                let bitmapHandle: windef::HBITMAP = unsafe { GetClipboardData(format) as *mut _ };
            }
            CF_DIB => {
                let bitmapInfo: *mut wingdi::BITMAPINFO =
                    unsafe { GetClipboardData(format) as *mut _ };
                globalPointers.push(unsafe {
                    winbase::GlobalAlloc(winbase::GHND, std::mem::size_of::<wingdi::BITMAPINFO>())
                });
                let memPointer: *mut winnt::VOID =
                    unsafe { winbase::GlobalLock(globalPointers[globalPointers.len() - 1]) };
            }
            CF_DIBV5 => {
                let bitmapV5Info: *mut wingdi::BITMAPV5HEADER =
                    unsafe { GetClipboardData(format) as *mut _ };
            }
            CF_DIF => {}
            CF_DSPBITMAP => {}
            CF_DSPENHMETAFILE => {}
            CF_DSPMETAFILEPICT => {}
            CF_DSPTEXT => {}
            CF_ENHMETAFILE => {}
            CF_GDIOBJFIRST => {}
            CF_GDIOBJLAST => {}
            CF_HDROP => {}
            CF_LOCALE => {}
            CF_MAX => {}
            CF_METAFILEPICT => {}
            CF_OEMTEXT => {}
            CF_OWNERDISPLAY => {}
            CF_PALETTE => {}
            CF_PENDATA => {}
            CF_PRIVATEFIRST => {}
            CF_PRIVATELAST => {}
            CF_RIFF => {}
            CF_TIFF => {}
            CF_TEXT => {}
            CF_SYLK => {}
            CF_UNICODETEXT => {}
            CF_WAVE => {}
            _ => todo!(),
        }
        unsafe { winuser::GetClipboardData(format) };
    }
}

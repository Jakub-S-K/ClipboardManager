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
        let amountOfFormats = unsafe{winuser::CountClipboardFormats()};
        let mut currentFormat = 0;
        for i in 0..amountOfFormats {
            currentFormat = unsafe{winuser::EnumClipboardFormats(currentFormat)};
            self.parseData(currentFormat);
        }

        unsafe{winuser::EmptyClipboard()};
        unsafe { winuser::CloseClipboard() };
    }
    fn parseData(&mut self, format: u32){
        use winuser::*;
        match format {
            CF_BITMAP => {
                let bitmapHandle: windef::HBITMAP__ = unsafe{std::mem::transmute<winuser::um::winnt::HANDLE, windef::HBITMAP__>(GetClipboardData(format))};
            
            }
            CF_DIB => {}
            CF_DIBV5 => {}
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
        unsafe{winuser::GetClipboardData(format)};
    }
}

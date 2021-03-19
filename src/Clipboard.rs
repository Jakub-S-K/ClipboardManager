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
    fn new(tempHWND: windef::HWND) -> self {
        ClipbaordHandler {
            clipboardType: 2,
            hwnd: tempHWND,
        }
    }
    pub fn update(&mut self) {
        unsafe { winuser::OpenClipboard(hwnd) };
        let amountOfFormats = winuser::CountClipboardFormats();
        let currentFormat = 0;
        for i in 0..amountOfFormats {
            currentFormat = winuser::EnumClipboardFormats(currentFormat);
            parseData(currentFormat);
        }

        winuser::EmptyClipboard();
        unsafe { winuser::CloseClipboard() };
    }
    fn parseData(&mut self, format :u32)
    {
        use winuser::*;
        match format {
            CF_BITMAP =>{},
            CF_DIB =>{},
            CF_DIBV5=>{},
            CF_DIF=>{},
            CF_DSPBITMAP=>{},
            CF_DSPENHMETAFILE=>{},
            CF_DSPMETAFILEPICT=>{},
            CF_DSPTEXT=>{},
            CF_ENHMETAFILE=>{},
            CF_GDIOBJFIRST=>{},
            CF_GDIOBJLAST=>{},
            CF_HDROP=>{},
            CF_LOCALE=>{},
            CF_MAX=>{},
            CF_METAFILEPICT=>{},
            CF_OEMTEXT=>{},
            CF_OWNERDISPLAY=>{},
            CF_PALETTE=>{},
            CF_PENDATA=>{},
            CF_PRIVATEFIRST=>{},
            CF_PRIVATELAST=>{},
            CF_RIFF=>{},
            CF_TIFF=>{},
            CF_TEXT=>{},
            CF_SYLK=>{},
            CF_UNICODETEXT=>{},
            CF_WAVE=>{},
            _ => todo!();
        }
        winuser::GetClipboardData(format);
        
    }
}

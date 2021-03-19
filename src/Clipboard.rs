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
        for i in amountOfFormats {
            currentFormat = winuser::EnumClipboardFormats(currentFormat);
            parseData(currentFormat);
        }

        winuser::EmptyClipboard();
        unsafe { winuser::CloseClipboard() };
    }
    fn parseData(&mut self, format :u32)
    {
        match format {
            winuser::
        }
        winuser::GetClipboardData(format);
        
    }
}

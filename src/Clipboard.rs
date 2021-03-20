#[allow(non_snake_case)]
use winapi::{
    shared::{minwindef, windef},
    um::{winbase, wingdi, winnt, winuser},
};

enum CLIPBOARDFORMATS {
    BITMAP(winnt::HANDLE),
    DIB(wingdi::BITMAPINFO),
    DIBV5(wingdi::BITMAPV5HEADER),
    DIF(),
    DSPBITMAP(),
    DSPENHMETAFILE(),
    DSPMETAFILEPICT(),
    DSPTEXT(),
    ENHMETAFILE(),
    GDIOBJFIRST(),
    GDIOBJLAST(),
    HDROP(),
    LOCALE(),
    MAX(),
    METAFILEPICT(),
    OEMTEXT(std::ffi::CString),
    OWNERDISPLAY(),
    PALETTE(),
    PENDATA(),
    PRIVATEFIRST(),
    PRIVATELAST(),
    RIFF(),
    TIFF(),
    TEXT(std::ffi::CString),
    SYLK(),
    UNICODETEXT(std::ffi::CString),
    WAVE(),
    HTML(std::ffi::CString),
}

struct ClipbaordEntity {
    format: Vec<CLIPBOARDFORMATS>,
}

#[allow(non_snake_case)]
pub struct ClipbaordHandler {
    hwnd: windef::HWND,
    data: Vec<Vec<ClipbaordEntity>>,
}

#[allow(non_snake_case)]
impl ClipbaordHandler {
    fn new(tempHWND: windef::HWND) -> Self {
        ClipbaordHandler {
            hwnd: tempHWND,
            data: Vec::new(),
        }
    }
    pub fn update(&mut self) {
        unsafe { winuser::OpenClipboard(self.hwnd) };
        let amountOfFormats = unsafe { winuser::CountClipboardFormats() };
        let mut currentFormat = 0;
        // to register history, get current clipboard in usage
        self.data.push(Vec::new());
        // to check which clipboard
        // this is the only existing vector
        self.data[0][0].format = Vec::new();
        for i in 0..amountOfFormats {
            currentFormat = unsafe { winuser::EnumClipboardFormats(currentFormat) };
            self.parseData(currentFormat, 0, 0, i as usize);
        }
        unsafe { winuser::EmptyClipboard() };
        unsafe { winuser::CloseClipboard() };
    }
    fn retrieveClipboardDataAs<T>(&self, format: u32) -> *mut T {
        unsafe { winuser::GetClipboardData(format) as *mut T }
    }
    fn parseData(
        &mut self,
        formatID: u32,
        clipboardIndex: usize,
        historyIndex: usize,
        formatIndex: usize,
    ) {
        use winuser::*;
        let globalPointer: winnt::HANDLE;
        let mut format = &mut self.data[clipboardIndex][historyIndex].format[formatIndex];
        match formatID {
            CF_BITMAP => {
                let bitmapHandle: windef::HBITMAP =
                    unsafe { winuser::GetClipboardData(formatID) as *mut _ };
            }
            CF_DIB => {
                format = &mut CLIPBOARDFORMATS::DIB(unsafe {
                    *self.retrieveClipboardDataAs::<wingdi::BITMAPINFO>(formatID)
                });
                globalPointer = unsafe {
                    winbase::GlobalAlloc(winbase::GHND, std::mem::size_of::<wingdi::BITMAPINFO>())
                };
                let memPointer: *mut winnt::VOID = unsafe { winbase::GlobalLock(globalPointer) };
            }
            CF_DIBV5 => {
                let bitmapV5Info: *mut wingdi::BITMAPV5HEADER =
                    unsafe { GetClipboardData(formatID) as *mut _ };
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
            _ => unimplemented!("This format is not supported"),
        }
    }
}

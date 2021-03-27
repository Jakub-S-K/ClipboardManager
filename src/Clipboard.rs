#[allow(non_snake_case)]
use winapi::{
    shared::windef,
    um::{winbase, wingdi, winnt, winuser},
};

#[allow(non_snake_case)]
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
    EMPTY,
}

#[allow(non_snake_case)]
impl CLIPBOARDFORMATS {
    //Get number behind enum, it will be used later for saving clipboards data in file
    pub fn getID(&self) -> u32 {
        match &*self {
            CF_BITMAP => 0,
            CF_DIB => 1,
            CF_DIBV5 => 2,
            CF_DIF => 3,
            CF_DSPBITMAP => 4,
            CF_DSPENHMETAFILE => 5,
            CF_DSPMETAFILEPICT => 6,
            CF_DSPTEXT => 7,
            CF_ENHMETAFILE => 8,
            CF_GDIOBJFIRST => 9,
            CF_GDIOBJLAST => 10,
            CF_HDROP => 11,
            CF_LOCALE => 12,
            CF_MAX => 13,
            CF_METAFILEPICT => 14,
            CF_OEMTEXT => 15,
            CF_OWNERDISPLAY => 16,
            CF_PALETTE => 17,
            CF_PENDATA => 18,
            CF_PRIVATEFIRST => 19,
            CF_PRIVATELAST => 20,
            CF_RIFF => 21,
            CF_TIFF => 22,
            CF_TEXT => 23,
            CF_SYLK => 24,
            CF_UNICODETEXT => 25,
            CF_WAVE => 26,
            EMPTY => 27,
            _ => panic!("Unsupported format"),
        }
    }
}

struct ClipboardEntity {
    format: Vec<CLIPBOARDFORMATS>,
}

impl ClipboardEntity {
    pub fn with_capacity(capacity: usize) -> Self {
        ClipboardEntity {
            format: Vec::with_capacity(capacity),
        }
    }
    pub fn new() -> Self {
        ClipboardEntity { format: Vec::new() }
    }
}

#[allow(non_snake_case)]
pub struct ClipboardHandler {
    hwnd: windef::HWND,
    data: Vec<Vec<ClipboardEntity>>,
    currentHistory: usize,
    currentClipboard: usize,
    maxHistorySize: usize,
    maxClipboardSize: usize,
}

#[allow(non_snake_case)]
impl ClipboardHandler {
    // TODO file that contains data, how much clipboards to create, how much history to store
    // TODO load from file past clipboards, and assign them to struct upon creation ClipboardHandler::Loadout
    fn new_default(tempHWND: windef::HWND) -> Self {
        let tempMaxClipSize: usize = 4;
        let tempMaxHistSize: usize = 5;
        let tempCurrentHist: usize = 0;
        let tempCurrentClip: usize = 0;
        let mut tempVec = Vec::with_capacity(tempMaxClipSize);
        for i in 0..tempMaxClipSize {
            tempVec.push(Vec::with_capacity(tempMaxHistSize));
            for _j in 0..tempMaxHistSize {
                tempVec[i as usize].push(ClipboardEntity::new());
            }
        }
        ClipboardHandler {
            hwnd: tempHWND,
            data: tempVec,
            currentClipboard: tempCurrentClip,
            currentHistory: tempCurrentHist,
            maxHistorySize: tempMaxHistSize,
            maxClipboardSize: tempMaxClipSize,
        }
    }
    fn new_loadout(tempHWND: windef::HWND) -> Option<Self> {
        let tempMaxClipSize: usize = 3;
        let tempMaxHistSize: usize = 5;
        let tempCurrentHist: usize = 0;
        let tempCurrentClip: usize = 0;
        let mut tempVec = Vec::with_capacity(tempMaxClipSize);
        for i in 0..tempMaxClipSize {
            tempVec.push(Vec::with_capacity(tempMaxHistSize));
            for _j in 0..tempMaxHistSize {
                tempVec[i as usize].push(ClipboardEntity::new());
            }
        }
        None //For now custom file format is not supported
    }
    pub fn new(tempHWND: windef::HWND) -> Self {
        match ClipboardHandler::new_loadout(tempHWND) {
            Some(clipboard) => clipboard,
            None => ClipboardHandler::new_default(tempHWND),
        }
    }
    #[inline]
    fn getCurrentFormat(&mut self) -> &mut ClipboardEntity {
        &mut self.data[self.currentClipboard][self.currentHistory]
    }
    pub fn update(&mut self) {
        unsafe { winuser::OpenClipboard(self.hwnd) };
        let amountOfFormats = unsafe { winuser::CountClipboardFormats() };
        let mut currentFormat = 0;
        // to register history, get current clipboard in usage
        self.data.push(Vec::new());
        // to check which clipboard
        // this is the only existing vector
        self.getCurrentFormat().format = Vec::new();
        for i in 0..amountOfFormats {
            currentFormat = unsafe { winuser::EnumClipboardFormats(currentFormat) };
            self.parseData(currentFormat, i as usize);
        }
        unsafe { winuser::CloseClipboard() };
    }
    fn retrieveClipboardDataAs<T>(&self, format: u32) -> *mut T {
        unsafe { winuser::GetClipboardData(format) as *mut T }
    }

    // implement for each type script running
    fn runScript(&self, format: usize) {}

    fn parseData(&mut self, formatID: u32, formatIndex: usize) {
        use winuser::*;
        let globalPointer: winnt::HANDLE;
        //let mut format = &mut self.getCurrentFormat().format[formatIndex];
        match formatID {
            CF_BITMAP => {
                if let CLIPBOARDFORMATS::BITMAP(data) = CLIPBOARDFORMATS::BITMAP(unsafe {
                    *self.retrieveClipboardDataAs::<winnt::HANDLE>(formatID)
                }) {
                    self.getCurrentFormat().format[formatIndex] = CLIPBOARDFORMATS::BITMAP(data);
                    self.runScript(formatIndex);
                    // TODO:
                    // get size from former clipboard
                    let mut size: u32 = 0;
                    match self.getCurrentFormat().format[0] {
                        CLIPBOARDFORMATS::DIB(data) => {
                            let bmi = data.bmiHeader;
                            let compresion = bmi.biCompression;
                            match compresion {
                                wingdi::BI_JPEG | wingdi::BI_PNG => {
                                    size = bmi.biSizeImage;
                                }
                                wingdi::BI_BITFIELDS | wingdi::BI_RGB => {
                                    let width = bmi.biWidth;
                                    let height = bmi.biHeight;
                                    size = (width * height * 3) as u32;
                                }
                                _ => {
                                    let width = bmi.biWidth;
                                    let mut height = bmi.biHeight;
                                    size = 0;
                                    if height < 0 {
                                        height = -height;
                                    }
                                    let bitCount = bmi.biBitCount;
                                    size = (width * height * bitCount as i32) as u32;
                                }
                            }
                        }
                        CLIPBOARDFORMATS::DIBV5(data) => {}
                        _ => unimplemented!(),
                    }
                    globalPointer = unsafe {
                        winbase::GlobalAlloc(winbase::GHND, std::mem::size_of::<winnt::HANDLE>())
                    };
                    let memPointer: *mut winnt::VOID =
                        unsafe { winbase::GlobalLock(globalPointer) };
                    unsafe { std::ptr::copy(data, memPointer as winnt::HANDLE, size as usize) }
                    unsafe { winbase::GlobalUnlock(globalPointer) };
                    //globalPointer
                }
            }
            CF_DIB => {
                if let CLIPBOARDFORMATS::DIB(mut data) = CLIPBOARDFORMATS::DIB(unsafe {
                    *self.retrieveClipboardDataAs::<wingdi::BITMAPINFO>(formatID)
                }) {
                    self.getCurrentFormat().format[formatIndex] = CLIPBOARDFORMATS::DIB(data);
                    self.runScript(formatIndex);
                    globalPointer = unsafe {
                        winbase::GlobalAlloc(
                            winbase::GHND,
                            std::mem::size_of::<wingdi::BITMAPINFO>(),
                        )
                    };
                    let memPointer: *mut winnt::VOID =
                        unsafe { winbase::GlobalLock(globalPointer) };
                    unsafe { std::ptr::copy(&mut data, memPointer as *mut wingdi::BITMAPINFO, 1) }
                    unsafe { winbase::GlobalUnlock(globalPointer) };
                    //globalPointer
                }
            }
            CF_DIBV5 => {
                if let CLIPBOARDFORMATS::DIBV5(mut data) = CLIPBOARDFORMATS::DIBV5(unsafe {
                    *self.retrieveClipboardDataAs::<wingdi::BITMAPV5HEADER>(formatID)
                }) {
                    self.getCurrentFormat().format[formatIndex] = CLIPBOARDFORMATS::DIBV5(data);
                    self.runScript(formatIndex);
                    globalPointer = unsafe {
                        winbase::GlobalAlloc(
                            winbase::GHND,
                            std::mem::size_of::<wingdi::BITMAPINFO>(),
                        )
                    };
                    let memPointer: *mut winnt::VOID =
                        unsafe { winbase::GlobalLock(globalPointer) };
                    unsafe {
                        std::ptr::copy(&mut data, memPointer as *mut wingdi::BITMAPV5HEADER, 1)
                    }
                    unsafe { winbase::GlobalUnlock(globalPointer) };
                    //globalPointer
                }
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
            EMPTY => {}
            _ => unimplemented!("This format is not supported"),
        }
    }

    // Funcitons to be called from eventHandler for script use only
    pub fn changeCurrentClipboard(&mut self, newClipbaord: usize) {
        if newClipbaord < self.maxClipboardSize {
            self.currentClipboard = newClipbaord;
        } else {
            println!("Picked clipboard out of scope");
        }
    }
}

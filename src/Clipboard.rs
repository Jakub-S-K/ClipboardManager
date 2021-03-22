#[allow(non_snake_case)]
use winapi::{
    shared::windef,
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
    EMPTY,
}

impl CLIPBOARDFORMATS { //Get number behind enum, it will be used later for saving clipboards data in file
    pub fn getID(&self) -> u32 {
        match *self {
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

struct ClipbaordEntity {
    format: Vec<CLIPBOARDFORMATS>,
}

impl ClipbaordEntity {
    pub fn with_capacity(capacity: usize) -> Self {
        ClipbaordEntity {
            format: Vec::with_capacity(capacity),
        }
    }
    pub fn new() -> Self {
        ClipbaordEntity { format: Vec::new() }
    }
}

#[allow(non_snake_case)]
pub struct ClipbaordHandler {
    hwnd: windef::HWND,
    data: Vec<Vec<ClipbaordEntity>>,
    currentHistory: usize,
    currentClipboard: usize,
    maxHistorySize: usize,
    maxClipboardSize: usize,
}

#[allow(non_snake_case)]
impl ClipbaordHandler {
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
                tempVec[i as usize].push(ClipbaordEntity::new());
            }
        }
        ClipbaordHandler {
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
                tempVec[i as usize].push(ClipbaordEntity::new());
            }
        }
        None //For now custom file format is not supported
    }
    pub fn new(tempHWND: windef::HWND) -> Self {
        match ClipbaordHandler::new_loadout(tempHWND) {
            Some(clipboard) => clipboard,
            None => ClipbaordHandler::new_default(tempHWND),
        }
    }
    #[inline]
    fn getCurrentFormat(&mut self) -> &mut ClipbaordEntity {
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
    fn parseData(&mut self, formatID: u32, formatIndex: usize) {
        use winuser::*;
        let globalPointer: winnt::HANDLE;
        let mut format = &mut self.getCurrentFormat().format[formatIndex];
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
            EMPTY => {}
            _ => unimplemented!("This format is not supported"),
        }
    }
}

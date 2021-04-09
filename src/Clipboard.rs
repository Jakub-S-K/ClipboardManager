use widestring::U16CString;
#[allow(non_snake_case)]
use widestring::WideCString;
use winapi::{
    shared::{minwindef, windef},
    um::{errhandlingapi, winbase, wingdi, winnt, winuser},
};

#[allow(non_snake_case)]
enum CLIPBOARDFORMATS {
    BITMAP(wingdi::BITMAP, Vec<u8>),
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
    OEMTEXT(String),
    OWNERDISPLAY(),
    PALETTE(),
    PENDATA(),
    PRIVATEFIRST(),
    PRIVATELAST(),
    RIFF(),
    TIFF(),
    TEXT(String),
    SYLK(),
    UNICODETEXT(String),
    WAVE(),
    EMPTY,
}

#[allow(non_snake_case)]
impl CLIPBOARDFORMATS {
    //Get number behind enum, it will be used later for saving clipboards data in file
    pub fn getID(&self) -> u32 {
        match *self {
            CLIPBOARDFORMATS::BITMAP(_, _) => 0,
            CLIPBOARDFORMATS::DIB(_) => 1,
            CLIPBOARDFORMATS::DIBV5(_) => 2,
            CLIPBOARDFORMATS::DIF() => 3,
            CLIPBOARDFORMATS::DSPBITMAP() => 4,
            CLIPBOARDFORMATS::DSPENHMETAFILE() => 5,
            CLIPBOARDFORMATS::DSPMETAFILEPICT() => 6,
            CLIPBOARDFORMATS::DSPTEXT() => 7,
            CLIPBOARDFORMATS::ENHMETAFILE() => 8,
            CLIPBOARDFORMATS::GDIOBJFIRST() => 9,
            CLIPBOARDFORMATS::GDIOBJLAST() => 10,
            CLIPBOARDFORMATS::HDROP() => 11,
            CLIPBOARDFORMATS::LOCALE() => 12,
            CLIPBOARDFORMATS::MAX() => 13,
            CLIPBOARDFORMATS::METAFILEPICT() => 14,
            CLIPBOARDFORMATS::OEMTEXT(_) => 15,
            CLIPBOARDFORMATS::OWNERDISPLAY() => 16,
            CLIPBOARDFORMATS::PALETTE() => 17,
            CLIPBOARDFORMATS::PENDATA() => 18,
            CLIPBOARDFORMATS::PRIVATEFIRST() => 19,
            CLIPBOARDFORMATS::PRIVATELAST() => 20,
            CLIPBOARDFORMATS::RIFF() => 21,
            CLIPBOARDFORMATS::TIFF() => 22,
            CLIPBOARDFORMATS::TEXT(_) => 23,
            CLIPBOARDFORMATS::SYLK() => 24,
            CLIPBOARDFORMATS::UNICODETEXT(_) => 25,
            CLIPBOARDFORMATS::WAVE() => 26,
            CLIPBOARDFORMATS::EMPTY => 27,
        }
    }
}

struct ClipboardEntity {
    format: CLIPBOARDFORMATS,
}

impl ClipboardEntity {
    pub fn new() -> Self {
        ClipboardEntity {
            format: CLIPBOARDFORMATS::EMPTY,
        }
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
        println!("update Function");
        let amountOfFormats = unsafe { winuser::CountClipboardFormats() };
        let mut currentFormat = 0;
        // to register history, get current clipboard in usage
        self.data.push(Vec::new());
        // to check which clipboard
        // this is the only existing vector

        unsafe { winuser::OpenClipboard(self.hwnd) };
        for _i in 0..amountOfFormats {
            currentFormat = unsafe { winuser::EnumClipboardFormats(currentFormat) };
            self.parseData(currentFormat);
            if let CLIPBOARDFORMATS::EMPTY = self.getCurrentFormat().format {
                continue;
            }
            break;
        }
    }
    // implement for each type script running
    fn runScript(&mut self) -> bool {
        match self.getCurrentFormat().format {
            _ => return false,
        }
    }

    fn parseData(&mut self, formatID: u32) {
        use winuser::*;
        match formatID {
            CF_BITMAP => {
                if unsafe { winuser::IsClipboardFormatAvailable(CF_DIB) } != 0 {
                    let data =
                        unsafe { *(winuser::GetClipboardData(CF_DIB) as *mut wingdi::BITMAPINFO) };
                    self.processBITMAPINFO(&data.bmiHeader);

                    data.bmiHeader.biWidth;
                } else if unsafe { winuser::IsClipboardFormatAvailable(CF_DIBV5) } != 0 {
                } else {
                    panic!("Chuj, nie dziala");
                }
                let a = unsafe { winuser::GetClipboardData(formatID) as *mut wingdi::BITMAP };
                println!("{}", unsafe { errhandlingapi::GetLastError() });
                let dupa = unsafe { *a }.bmBitsPixel;
                let tempBitmap: wingdi::BITMAP = unsafe {
                    *std::mem::transmute::<_, &wingdi::BITMAP>(winuser::GetClipboardData(formatID))
                };
                let tempSizeOfPointer =
                    (tempBitmap.bmHeight * tempBitmap.bmWidth * tempBitmap.bmBitsPixel as i32)
                        as usize;
                let tempPointer: &[u8] = unsafe {
                    std::slice::from_raw_parts(tempBitmap.bmBits as *mut u8, tempSizeOfPointer)
                };
                let mut tempVec: Vec<u8> = Vec::new();
                tempVec.extend_from_slice(tempPointer);
                let mut finalBitmap = tempBitmap;
                finalBitmap.bmBits = tempVec.as_mut_ptr() as *mut core::ffi::c_void;
                self.getCurrentFormat().format = CLIPBOARDFORMATS::BITMAP(finalBitmap, tempVec);
                // check for avaliable CF_DIB or CF_DIBV5
                if self.runScript() {
                    let memPointer = unsafe {
                        winbase::GlobalAlloc(winbase::GHND, std::mem::size_of::<wingdi::BITMAP>())
                    };
                    let lockedMem = unsafe { winbase::GlobalLock(memPointer) };
                    if let CLIPBOARDFORMATS::BITMAP(data, _pointer) =
                        &self.getCurrentFormat().format
                    {
                        unsafe { std::ptr::copy(&data, lockedMem as *mut &wingdi::BITMAP, 1) };
                    }
                    unsafe { winbase::GlobalUnlock(memPointer) };
                    unsafe { winuser::EmptyClipboard() };
                    unsafe { winuser::SetClipboardData(formatID, memPointer) };
                    unsafe { winuser::CloseClipboard() };
                }
            }
            CF_DIB => {
                // check for avaliable CF_BITMAP
            }
            CF_DIBV5 => {
                // check for avaliable CF_BITMAP
            }
            CF_LOCALE => {}
            CF_MAX => {}
            CF_METAFILEPICT => {}
            CF_OEMTEXT => {
                let tempText =
                    unsafe { winuser::GetClipboardData(formatID) as *mut std::os::raw::c_char };
                let data = unsafe {
                    std::ffi::CStr::from_ptr(tempText)
                        .to_string_lossy()
                        .into_owned()
                };
                self.getCurrentFormat().format = CLIPBOARDFORMATS::OEMTEXT(data);
                // run script and make it into global mem
                //if self.runScript()
                {
                    if let CLIPBOARDFORMATS::OEMTEXT(data) = &self.getCurrentFormat().format {
                        let memSize: usize = data.len();
                        let memPointer = unsafe { winbase::GlobalAlloc(winbase::GHND, memSize) };
                        let lockedMem = unsafe { winbase::GlobalLock(memPointer) };
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                data.as_ptr(),
                                lockedMem as *mut u8,
                                memSize,
                            )
                        };
                        unsafe { winbase::GlobalUnlock(memPointer) };

                        unsafe { winuser::EmptyClipboard() };
                        unsafe { winuser::SetClipboardData(CF_TEXT, memPointer) };
                        unsafe { winuser::CloseClipboard() };
                    }
                }
            }
            CF_OWNERDISPLAY => {}
            CF_PALETTE => {}
            CF_PENDATA => {}
            CF_PRIVATEFIRST => {}
            CF_PRIVATELAST => {}
            CF_RIFF => {}
            CF_TIFF => {}
            CF_TEXT => {
                let tempText =
                    unsafe { winuser::GetClipboardData(formatID) as *mut std::os::raw::c_char };
                let data = unsafe {
                    std::ffi::CStr::from_ptr(tempText)
                        .to_string_lossy()
                        .into_owned()
                };
                self.getCurrentFormat().format = CLIPBOARDFORMATS::TEXT(data);
                //if self.runScript()
                {
                    if let CLIPBOARDFORMATS::TEXT(data) = &self.getCurrentFormat().format {
                        let memSize: usize = data.len();
                        let memPointer = unsafe { winbase::GlobalAlloc(winbase::GHND, memSize) };
                        let lockedMem = unsafe { winbase::GlobalLock(memPointer) };
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                data.as_ptr(),
                                lockedMem as *mut u8,
                                memSize,
                            )
                        };
                        unsafe { winbase::GlobalUnlock(memPointer) };

                        unsafe { winuser::EmptyClipboard() };
                        unsafe { winuser::SetClipboardData(CF_TEXT, memPointer) };
                        unsafe { winuser::CloseClipboard() };
                    }
                }
                // run script and make it into global mem
            }
            CF_SYLK => {}
            CF_UNICODETEXT => {
                let data = unsafe {
                    U16CString::from_raw(winuser::GetClipboardData(formatID) as *mut u16)
                };
                self.getCurrentFormat().format =
                    CLIPBOARDFORMATS::UNICODETEXT(data.to_string().unwrap());
                // run script and make it into global mem
                //if self.runScript()
                {
                    if let CLIPBOARDFORMATS::UNICODETEXT(data) = &self.getCurrentFormat().format {
                        let memSize: usize = data.len();
                        let memPointer = unsafe { winbase::GlobalAlloc(winbase::GHND, memSize) };
                        let lockedMem = unsafe { winbase::GlobalLock(memPointer) };
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                data.as_ptr(),
                                lockedMem as *mut u8,
                                memSize,
                            )
                        };
                        unsafe { winbase::GlobalUnlock(memPointer) };

                        unsafe { winuser::EmptyClipboard() };
                        unsafe { winuser::SetClipboardData(CF_TEXT, memPointer) };
                        unsafe { winuser::CloseClipboard() };
                    }
                }
            }
            CF_WAVE => {}
            EMPTY => {}
            _ => unimplemented!("This format is not supported"),
        }
    }

    fn processBITMAPINFO(&mut self, data: &wingdi::BITMAPINFOHEADER) -> usize {
        match data.biCompression {
            wingdi::BI_RGB => self.processBitAmountSub(
                (data.biWidth * i32::abs(data.biHeight)) as f32
                    * ((data.biBitCount as f32) * 0.125),
            ),
            wingdi::BI_RLE8 => {0}
            wingdi::BI_RLE4 => {0}
            wingdi::BI_BITFIELDS => self.processBitAmountSub(
                (data.biWidth * i32::abs(data.biHeight)) as f32
                    * ((data.biBitCount as f32) * 0.125),
            ),
            wingdi::BI_JPEG => data.biSizeImage as usize,
            wingdi::BI_PNG => data.biSizeImage as usize,
            _ => panic!("I shouldn't be here"),
        }
    }
    fn processBitAmountSub(&self, bytes: f32) -> usize {
        if bytes - ((bytes as i32) as f32) < 1.0 {
            (bytes + 1.0) as usize
        } else {
            bytes as usize
        }
    }
    fn processBitAmountMod(&self, bytes: i32) -> f32 {
        let modVal = bytes % 8;
        if modVal != 0 {
            (bytes + (8 - modVal)) as f32
        } else {
            bytes as f32
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

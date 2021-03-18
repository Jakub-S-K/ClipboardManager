#[allow(non_snake_case)]
extern crate memory_module_sys;
#[macro_use]
extern crate sciter;

use std::ptr::null_mut;
use winapi::{
    shared::{minwindef, windef},
    um::{libloaderapi, wingdi, winuser},
};

#[allow(non_snake_case)]
mod EventHandler;
#[allow(non_snake_case)]
mod WinHandle;
#[allow(non_snake_case)]
mod windowAlingment;

use windowAlingment::*;

static archive: &[u8] = include_bytes!("../dupa.rc");

#[allow(non_snake_case)]
pub fn main() {
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true));

    let windowAlingment = WindowPos::new(
        getDesktopResolution(),
        45,
        15_f32,
        43_f32,
        WINDOWALINGMENT::BottomLeft,
    );
    let windowHwnd = unsafe {
        createWindow(windowProc, windowAlingment).expect("Nie udało się stworzyć okna")
    };
    unsafe { winuser::AddClipboardFormatListener(windowHwnd) };
    let mut frame = sciter::Window::attach(windowHwnd as sciter::types::HWINDOW);
    let eventcos = EventHandler::EventHandler { root: None };
    frame.event_handler(eventcos);
    frame.archive_handler(archive).unwrap();
    //frame.load_html(binHtml, Some("this://app/index.htm"));
    frame.load_file("this://app/index.htm");
    unsafe {
        winuser::ShowWindow(windowHwnd, winuser::SW_SHOW);
        let mut msg: winuser::MSG = std::mem::zeroed();
        while winuser::GetMessageA(&mut msg, null_mut(), 0, 0) != 0 {
            winuser::TranslateMessage(&msg);
            winuser::DispatchMessageA(&msg);
        }
    }
}

/*extern "system" fn HostCallbackFnc(scn: sciter::types::LPSCITER_CALLBACK_NOTIFICATION, callbackParam: sciter::types::LPVOID) -> u32
{
    match std::mem::transmute::<u32, sciter::types::SCITER_NOTIFICATION>((*scn).code) //cast from u32 to enum
    {
        sciter::types::SCITER_NOTIFICATION::SC_LOAD_DATA =>
        {

        }
        sciter::types::SCITER_NOTIFICATION::SC_DATA_LOADED =>
        {

        }
        sciter::types::SC
        _ => todo!();
    }
    unimplemented!
}*/

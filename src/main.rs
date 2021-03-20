extern crate memory_module_sys;
#[macro_use]
extern crate sciter;

use std::ptr::null_mut;

mod winapi_processes;

#[allow(non_snake_case)]
mod Event;
use Event::*;
#[allow(non_snake_case)]
mod Windows;
use Windows::*;


static archive: &[u8] = include_bytes!("../dupa.rc");

#[allow(non_snake_case)]
pub fn main() {
    winapi_processes::isProcessRunning("Telegram.exe");
    sciter::set_options(sciter::RuntimeOptions::DebugMode(true));
    let windowHandle = unsafe {
        WinHandler::new(
            "clipbaord_manager\0".as_bytes(),
            "Dupa\0".as_bytes(),
            WindowPos::new(45, 15_f32, 43_f32, WINDOWALINGMENT::TopRight),
        )
    };
    unsafe { windowHandle.hookClipboardListener() };
    let mut frame = sciter::Window::attach(windowHandle.getHWND() as sciter::types::HWINDOW);
    let eventcos = EventHandler { root: None, number:2};
    frame.event_handler(eventcos);
    frame.archive_handler(archive).unwrap();
    
    frame.load_file("this://app/index.htm");
    unsafe { windowHandle.messageLoop() };
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

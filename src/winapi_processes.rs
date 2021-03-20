use winapi::{
    shared::minwindef,
    um::{tlhelp32, winnt}
};

use std::ptr::null_mut;

pub fn isProcessRunning(name: &str) -> bool {

    let mut entry: tlhelp32::PROCESSENTRY32 = unsafe {std::mem::zeroed()};
    entry.dwSize = std::mem::size_of::<tlhelp32::PROCESSENTRY32>() as u32;

    let snapshot: winnt::HANDLE = unsafe {tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, 0)};
    unsafe {
        if tlhelp32::Process32First(snapshot, &mut entry) == minwindef::TRUE {
            //if entry.szExeFile == "target.exe" {
                todo!();
            //}
        }
    }
    
    true
}
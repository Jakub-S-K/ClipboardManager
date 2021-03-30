use winapi::{
    shared::minwindef,
    um::{tlhelp32, winnt},
};

use std::ptr::null_mut;

pub fn isProcessRunning(name: &str) -> bool {
    let mut entry: tlhelp32::PROCESSENTRY32 = unsafe { std::mem::zeroed() };
    entry.dwSize = std::mem::size_of::<tlhelp32::PROCESSENTRY32>() as u32;

    let snapshot: winnt::HANDLE =
        unsafe { tlhelp32::CreateToolhelp32Snapshot(tlhelp32::TH32CS_SNAPPROCESS, 0) };
    unsafe {
        if tlhelp32::Process32First(snapshot, &mut entry) == minwindef::TRUE {
            while tlhelp32::Process32Next(snapshot, &mut entry) != 0 {
                let u8_cast = &*(getNameSlice(&entry.szExeFile).unwrap() as *const [i8] as *const [u8]);
                if name.as_bytes() == u8_cast {
                    return true;
                }
                entry.szExeFile = std::mem::zeroed();
            }
        }
    }
    false
}

fn getNameSlice(array: &[i8; 260]) -> Option<&[i8]> {
    for (i, x) in array.iter().enumerate() {
        if *x == '\0' as i8 {
            return Some(&array[0..i]);
        }
    }
    return None;
}

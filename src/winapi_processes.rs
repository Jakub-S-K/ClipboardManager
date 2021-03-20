use winapi::{
    shared::minwindef,
    um::{psapi, winnt, processthreadsapi, handleapi, errhandlingapi},
};

use std::ptr::null_mut;

pub fn isProcessRunning(name: &str) -> bool {
    let mut processes: [u32; 1024] = unsafe {std::mem::zeroed()};
    let mut neededCnt: u32  = 0;
    let mut processCnt: u32;
    unsafe {
        if psapi::EnumProcesses(
            processes.as_mut_ptr(),
            std::mem::size_of_val(&processes) as _,
            &mut neededCnt,
        ) == 0
        {
            panic!("winapi_processes.rc | 17");
        }
    }
    processCnt = neededCnt / std::mem::size_of::<u32>() as u32;

    for i in 0..processCnt {
        if processes[i as usize] != 0 {
            println!("PID: {}", processes[i as usize]); //DEBUG
            
            match getProcessName(processes[i as usize]) {
                Some(val) => {
                    if val == name {
                        return true;
                    } else {
                        return false;
                    }
                }
                None => {
                    println!("Brak uprawnień");
                }
            } //uncomment this if getProcesName is inmplemented
        }
    }

    false
}

fn getProcessName(pid: u32) -> Option<String> {
    const SIZE: usize = 256;
    //let mut name = String::with_capacity(SIZE);
    let mut name: [winapi::ctypes::c_char; SIZE] = unsafe {std::mem::zeroed()};

    let hProcess: winnt::HANDLE = unsafe {processthreadsapi::OpenProcess(winnt::PROCESS_QUERY_LIMITED_INFORMATION | winnt::PROCESS_VM_READ, minwindef::FALSE, pid)};
    println!("hProcess: {}", hProcess as u64);
    if hProcess != null_mut() {
        let mut hMod: minwindef::HMODULE = unsafe {std::mem::zeroed()};
        let mut neededCnt: u32 = 0;

        unsafe {
            if psapi::EnumProcessModules(hProcess, &mut hMod, std::mem::size_of::<minwindef::HMODULE>() as u32, &mut neededCnt) != 0 {
                psapi::GetModuleBaseNameA(hProcess, hMod, name.as_mut_ptr(), SIZE as u32);
            }
        }
    } else {
        println!("ERROR: {}", unsafe{errhandlingapi::GetLastError()});
        return None
    }
    unsafe {handleapi::CloseHandle(hProcess)};
    
    unsafe {return Some(String::from_raw_parts(std::mem::transmute::<*mut i8, *mut u8>(name.as_mut_ptr()), SIZE, SIZE))};
}

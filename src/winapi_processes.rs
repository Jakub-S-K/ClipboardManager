use winapi::{
    shared::minwindef,
    um::{psapi, winnt, processthreadsapi},
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
            
            if getProcessName(processes[i as usize]).unwrap().as_str() == name {
              return true
            } //uncomment this if getProcesName is inmplemented
        }
    }

    false
}

fn getProcessName(pid: u32) -> Option<String> {
    const SIZE: usize = 256;
    let mut name = String::with_capacity(SIZE);

    let hProcess = unsafe {processthreadsapi::OpenProcess(winnt::PROCESS_QUERY_INFORMATION | winnt::PROCESS_VM_READ, minwindef::FALSE, pid)};
    
    if hProcess != null_mut() {
        let mut hMod: minwindef::HMODULE;
        let mut neededCnt: u32 = 0;

        unsafe {
            todo!();
            //if psapi::EnumProcessModules(hProcess, &mut hMod, std::mem::size_of::<minwindef::HMODULE>() as u32, &mut neededCnt) != 0 {
            //    psapi::GetModuleBaseNameA(hProcess, hMod, name.as_mut_ptr() as *mut i8, SIZE as u32, );
            //}
        }
    }

    Some(name)
}

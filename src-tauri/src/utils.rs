use std::{ffi::CString, fs::File, io::{self, Seek, SeekFrom, Write}, os::raw::c_void, path::PathBuf, ptr::null_mut};

use windows::{Win32::{Foundation::{CloseHandle, HANDLE}, System::{Diagnostics::Debug::WriteProcessMemory, LibraryLoader::{GetModuleHandleA, GetProcAddress}, Memory::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE, VirtualAllocEx, VirtualFreeEx}, Threading::{LPTHREAD_START_ROUTINE, OpenProcess, PROCESS_ALL_ACCESS, PROCESS_QUERY_INFORMATION, PROCESS_SUSPEND_RESUME}}}, core::PCSTR};

#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateRemoteThread(hProcess: HANDLE, lpThreadAttributes: *mut c_void, dwStackSize: usize, lpStartAddress: LPTHREAD_START_ROUTINE, lpParameter: *mut c_void, dwCreationFlags: u32, lpThreadId: *mut u32) -> HANDLE;
}

#[link(name = "ntdll")]
unsafe extern "system" {
    fn NtSuspendProcess(proc: HANDLE) -> i32;
}

pub fn nt_suspend_process(pid: u32) -> bool {
    unsafe {
        let handle = OpenProcess(
            PROCESS_SUSPEND_RESUME | PROCESS_QUERY_INFORMATION,
            false,
            pid
        ).unwrap();

        if handle.is_invalid() {
            return false;
        }

        let status = NtSuspendProcess(handle);
        let _ = CloseHandle(handle);

        status == 0
    }
}
    
// This is directly skidded from the original project
pub fn inject_dll(pid: u32, dll_path: &str) -> Result<(), &'static str> {
    if !PathBuf::from(dll_path).is_file() {
        return Err("DLL not found!");
    }
    
    let path_size = dll_path.len();

    unsafe {
        let handle = OpenProcess(
            PROCESS_ALL_ACCESS, 
            false, 
            pid
        ).unwrap();

        if handle.is_invalid() {
            return Err("Invalid Handle");
        }

        let mem_loc = VirtualAllocEx(
            handle, 
            None, 
            path_size, 
            MEM_COMMIT | MEM_RESERVE, 
            PAGE_READWRITE
        );

        let write_process_memory = WriteProcessMemory(
            handle, 
            mem_loc, 
            CString::new(dll_path).unwrap().as_ptr() as *mut c_void, 
            path_size, 
            None
        );
        match write_process_memory {
            Ok(_) => {},
            Err(_) => {
                return Err("WriteProcessMemory failed!");
            }
        }
        
        let kernel32_dll_name = CString::new("kernel32.dll").unwrap();
        let kernel32_handle = GetModuleHandleA(PCSTR::from_raw(kernel32_dll_name.as_ptr() as *const u8));

        let loadlibrary_name = CString::new("LoadLibraryA").unwrap();

        let load_library_addr = GetProcAddress(kernel32_handle.unwrap(), PCSTR::from_raw(loadlibrary_name.as_ptr() as *const u8));
        if load_library_addr.is_none() {
            return Err("LoadLibraryA failed");
        }
        let load_library = load_library_addr.unwrap();

        let load_library_fn: LPTHREAD_START_ROUTINE = std::mem::transmute(load_library);

        let thread_handle = CreateRemoteThread(
            handle, 
            null_mut(), 
            0, 
            load_library_fn, 
            mem_loc, 
            0, 
            null_mut()
        );
        if thread_handle.is_invalid() {
            let _ = VirtualFreeEx(
                handle, 
                mem_loc, 
                0, 
                MEM_RELEASE
            );
            return Err("Thread Handle Invalid");
        }
    }

    Ok(())
}

#[allow(dead_code)]
pub fn apply_patch(file: &mut File, pos: u64, data: &[u8]) -> Result<(), io::Error> {
    match file.seek(SeekFrom::Start(pos)) {
        Ok(_) => {},
        Err(e) => {
            return Err(e);
        }
    }

    match file.write_all(data) {
        Ok(_) => {},
        Err(e) => {
            return Err(e);
        }
    }

    Ok(())
}

pub fn patch_uefn(path: PathBuf) -> Result<(), String> {
    let mut file = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path) {
            Ok(c) => {
                c
            },
            Err(e) => {
                let e_str: String = e.to_string();
                return Err(e_str);
            }
        };

    // "Cannot modify cooked assets" patch
    // 162051E3 : Change 32 C0 to 90 90
    match apply_patch(&mut file, 0x162051E3, &[0x90, 0x90]) {
        Ok(_) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    // Unable to edit cooked asset
    // 95C3A55 : 32 DB -> B3 01
    match apply_patch(&mut file, 0x95C3A55, &[0xB3, 0x01]) {
        Ok(_) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    // Copy Objects
    // 9352C78 : 0F 85 52 01 00 00 -> 90 90 90 90 90 90
    match apply_patch(&mut file, 0x9352C78, &[0x90, 0x90, 0x90, 0x90, 0x90, 0x90]) {
        Ok(_) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    // Package is cooked or missing editor data
    // 935339D : 74 -> EB
    match apply_patch(&mut file, 0x935339D, &[0xEB]) {
        Ok(_) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    // Fixed additive animations
    // ABBF943 : 74 -> 75
    match apply_patch(&mut file, 0xABBF943, &[0x75]) {
        Ok(_) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    // Commandline Args
    // C4E735C : 75 -> EB
    match apply_patch(&mut file, 0xC4E735C, &[0xEB]) {
        Ok(_) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    }

    Ok(())
}

pub fn patch_for_server(path: &PathBuf) -> Result<(), String> {
    let mut file = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path) {
            Ok(c) => {
                c
            },
            Err(e) => {
                let e_str: String = e.to_string();
                return Err(e_str);
            }
        };

    const PATCHED_HEADLESS: &[u8] = &[
        45, 0, 108, 0, 111, 0, 103, 0, 32, 0, 45, 0, 110, 0, 111, 0,
        115, 0, 112, 0, 108, 0, 97, 0, 115, 0, 104, 0, 32, 0, 45, 0,
        110, 0, 111, 0, 115, 0, 111, 0, 117, 0, 110, 0, 100, 0, 32, 0,
        45, 0, 110, 0, 117, 0, 108, 0, 108, 0, 114, 0, 104, 0, 105, 0,
        32, 0, 45, 0, 117, 0, 115, 0, 101, 0, 111, 0, 108, 0, 100, 0,
        105, 0, 116, 0, 101, 0, 109, 0, 99, 0, 97, 0, 114, 0, 100, 0,
        115, 0, 32, 0, 32, 0, 32, 0, 32, 0, 32, 0, 32, 0, 32, 0
    ];

    // Patch some args I think
    match apply_patch(&mut file, 0xC23D69C, PATCHED_HEADLESS) {
        Ok(_) => {},
        Err(e) => {
            return Err(e.to_string());
        }
    };

    Ok(())
}
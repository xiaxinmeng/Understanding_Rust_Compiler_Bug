rust
use std::{ffi::OsString, mem::{size_of, MaybeUninit}, os::windows::ffi::OsStringExt, path::Path};

use winapi::{
    shared::{minwindef::{HMODULE, LPVOID}, ntdef::NULL},
    um::{
        libloaderapi::GetModuleFileNameW, memoryapi::VirtualQuery, sysinfoapi::GetSystemInfo,
        winnt::MEMORY_BASIC_INFORMATION,
    },
};

unsafe fn print_loaded_images() {
    let mut system_info = MaybeUninit::zeroed().assume_init();
    GetSystemInfo(&mut system_info);

    let mut buf: Vec<u16> = Vec::with_capacity(1000);
    let mut memory_basic_info = MaybeUninit::zeroed().assume_init();

    let start = system_info.lpMinimumApplicationAddress;
    let end = system_info.lpMaximumApplicationAddress;
    let mut base = start;
    while base < end {
        VirtualQuery(base, &mut memory_basic_info, size_of::<MEMORY_BASIC_INFORMATION>());
        if base != NULL && memory_basic_info.AllocationBase == base {
            let len = GetModuleFileNameW(base as HMODULE, buf.as_mut_ptr(), 1000) as usize;
            if len > 0 {
                buf.set_len(len);
                let filename = OsString::from_wide(&buf[..]);
                let path = Path::new(&filename);
                println!("base: {:x} module: {} ", base as usize, path.display());
            }
        }

        base = ((base as usize) + memory_basic_info.RegionSize) as LPVOID;
    }
}

fn main() {
    unsafe { print_loaded_images() };
}

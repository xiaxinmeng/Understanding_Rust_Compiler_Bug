 rust
extern crate libc;

use std::ptr;
use std::io::IoError;

fn main() {
    for name in [&['a' as u16, 0xD83D, 0xDCA9, 'b' as u16, 0],
                 &['a' as u16, 0xD83D,         'b' as u16, 0]].iter() {
        let handle = unsafe {
            libc::CreateFileW(name.as_ptr(),
                              libc::FILE_GENERIC_WRITE,
                              0,
                              ptr::mut_null(),
                              libc::CREATE_ALWAYS,
                              libc::FILE_ATTRIBUTE_NORMAL,
                              ptr::mut_null())
        };
        let is_invalid = handle == libc::INVALID_HANDLE_VALUE as libc::HANDLE;
        println!("{} {} {}", name, is_invalid, IoError::last_error())
    }
}


PS C:\Users\micha\testexe> cat .\src\main.rs
use std::env;
use winapi::um::processenv::GetCommandLineW;
use std::os::windows::ffi::OsStringExt;
use std::ffi::OsString;
use std::slice;
fn main() {
    println!("env::args() = {:?}", env::args().collect::<Vec<_>>());
    let get_command_line_w = unsafe {
        let wide_ptr = GetCommandLineW();
        let mut wide_len = 0;
        while *wide_ptr.offset(wide_len) != 0 {
            wide_len += 1;
        }
        slice::from_raw_parts(wide_ptr, wide_len as usize)
    };
    println!("GetCommandLineW = {}", OsString::from_wide(get_command_line_w).to_str().unwrap());
}
PS C:\Users\micha\testexe>

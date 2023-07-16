 rust
static BUF_BYTES: uint = 4096;

#[cfg(unix)]
pub fn getcwd() -> Option<Path> {
    use std::c_str::CString;
    use libc::{c_char, size_t};

    let mut buf = [0 as c_char, ..BUF_BYTES];
    unsafe {
        if libc::getcwd(buf.as_mut_ptr(), buf.len() as size_t).is_null() {
            None
        } else {
            Some(Path::new(CString::new(buf.as_ptr(), false)))
        }
    }
}

#[cfg(windows)]
pub fn getcwd() -> Option<Path> {
    use libc::DWORD;
    use libc::GetCurrentDirectoryW;

    let mut buf = [0 as u16, ..BUF_BYTES];
    unsafe {
        if libc::GetCurrentDirectoryW(buf.len() as DWORD, buf.as_mut_ptr()) == 0 {
            return None;
        }
    }
    match String::from_utf16(std::str::truncate_utf16_at_nul(buf)) {
        Some(s) => Some(Path::new(s)),
        _ => None,
    }
}

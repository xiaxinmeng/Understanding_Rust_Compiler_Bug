 rust
#[cfg(unix)]
pub fn real_path(p: Path) -> Path {
    use libc::{c_char};
    use std::c_str::{CString};
    extern {
        fn realpath(path: *const c_char, resolved: *mut c_char) -> *const c_char;
    }
    let mut p = p.into_vec();
    p.push(0);
    let new_p = unsafe { realpath(p.as_ptr() as *const c_char, 0 as *mut c_char) };
    unsafe { Path::new(CString::new(new_p, true).as_bytes_no_nul()) }
}

#[cfg(windows)]
pub fn real_path(p: Path) -> Path {
    // TODO
    p
}

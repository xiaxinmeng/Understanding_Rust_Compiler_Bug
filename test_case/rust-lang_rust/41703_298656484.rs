rust
use std::ffi::{CStr, OsStr};
#[cfg(unix)] use std::os::unix::ffi::OsStrExt;

#[cfg(unix)]
fn cstr_to_path(s: &CStr) -> Option<&Path> {
    Path::new(OsStr::from_bytes(s.to_bytes()))
}

#[cfg(windows)]
fn cstr_to_path(s: &CStr) -> Option<&Path> {
    s.map(|s| Path::new(OsStr::new(s)))
}

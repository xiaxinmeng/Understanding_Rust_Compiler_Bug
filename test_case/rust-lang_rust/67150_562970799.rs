rust
use std::ffi::OsStr;

pub fn foo(s: Option<&OsStr>) -> bool {
    let so = OsStr::new("so");
    s.map_or(false, |x| x == so)
}

rust
use std::ffi::OsStr;
use std::os::ext::ffi::OsStrExt;

fn main() {
    OsStr::new("1234").encode_wide();
}

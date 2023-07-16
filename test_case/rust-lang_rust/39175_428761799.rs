rust
use std::ffi::OsString;

fn main() {
    let _ = OsString::from_wide(b"a\x00b\x00c\x00");
}

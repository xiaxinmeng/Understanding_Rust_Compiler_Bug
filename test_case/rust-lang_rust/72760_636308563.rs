rust
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::ffi::{OsStr, OsString};

fn main() {
    let mut base: Vec<u16> = OsStr::new("aé ").encode_wide().collect();
    base.push(0xD83D);
    let mut _res: Vec<u16> = OsString::from_wide(&base).encode_wide().collect();
}

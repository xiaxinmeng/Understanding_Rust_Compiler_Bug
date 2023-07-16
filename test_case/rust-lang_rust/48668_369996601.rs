rust
use std::path::Component;
use std::ffi::OsStr;

fn main() {
    let x: Box<OsStr> = Component::CurDir.as_ref().into();
}

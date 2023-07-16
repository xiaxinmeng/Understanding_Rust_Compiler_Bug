Rust
use std::ffi::OsStr;
use std::path::Path;
use std::thread;

fn main() {}

fn start(p: String) {
    thread::spawn(move || match Path::new(&p).extension() {
        Some(OsStr::new("ogg")) => {}
        _ => {}
    });
}

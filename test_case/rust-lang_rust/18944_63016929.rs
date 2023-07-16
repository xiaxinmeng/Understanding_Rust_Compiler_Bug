 rust
// otherargv.rs

extern crate libc;
use std::ptr;

fn main() {
    let args = std::os::args();
    println!("{}", args);

    match args.len() {
        1 => unsafe {
            // call itself with a custom argv.
            libc::execv("./otherargv\0".as_ptr() as *const i8,
                        ["abc\0".as_ptr() as *const i8,
                         "def\0".as_ptr() as *const i8,
                         ptr::null()].as_mut_ptr());

        },
        2 => unsafe {
            // call itself with an empty argv.
            libc::execv("./otherargv\0".as_ptr() as *const i8,
                        [ptr::null()].as_mut_ptr());
        },
        _ => {}
    }
}

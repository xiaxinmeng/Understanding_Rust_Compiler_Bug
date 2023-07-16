rust
#![feature(rustc_private)]
#![feature(lang_items)]
#![feature(start)]
#![no_std]

extern crate libc;

use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { libc::write(1, s.as_ptr().cast(), s.len()) };
        Ok(())
    }
}

#[start]
fn main(_: isize, _: *const *const u8) -> isize {
    let s = "world";
    writeln!(Stdout, "Hello, {}!", s).is_err() as isize
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { libc::abort() };
}

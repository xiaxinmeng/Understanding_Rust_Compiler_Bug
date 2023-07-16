rust
#![no_std]
#![feature(fmt_internals)]
#![feature(compiler_builtins)]
#![compiler_builtins]

use core::fmt;

struct String;

impl core::fmt::Write for String {
    fn write_str(&mut self, _: &str) -> fmt::Result {
        Ok(())
    }
}

fn x<T>(val: &T) {
    let mut buf = String;
    let mut formatter = core::fmt::Formatter::new(&mut buf);
    core::fmt::Display::fmt(val, &mut formatter).unwrap();
}

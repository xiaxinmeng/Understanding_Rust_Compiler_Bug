rust
use std::fmt::{self, Write};

struct Stdout;

extern "C" {
    fn write(fd: i32, buf: *const u8, count: usize) -> i32;
}

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe { write(1, s.as_ptr(), s.len()) };
        Ok(())
    }
}

pub fn test() {
    let _ = write!(Stdout, "Hello, {}!", "world");
}

 rust
#![crate_type="lib"]
use std::fmt::{self, Write};

pub fn test_write(c: char, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", c)
}

pub fn test_write_char(c: char, f: &mut fmt::Formatter) -> fmt::Result {
    f.write_char(c)
}

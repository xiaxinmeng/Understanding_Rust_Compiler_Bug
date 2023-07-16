rust
use std::io;

pub fn foo() -> io::Result<i32> {
    Err(io::Error::new(
        io::ErrorKind::Other,
        format!("Error!").as_ref(),
    ))
}

fn main() {}


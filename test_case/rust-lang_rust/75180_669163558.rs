rust
use std::{fmt, error::Error};

#[derive(Debug)]
pub struct MyError;

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "my error")
    }
}

impl Error for MyError {}

fn is_err<T: Error>() {}

fn main() {
    is_err::<&MyError>();
    is_err::<&dyn Error>();
    is_err::<&(dyn Error + 'static)>();
}

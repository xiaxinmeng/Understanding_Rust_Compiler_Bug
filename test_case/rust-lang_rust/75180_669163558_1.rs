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
impl<'a> Error for &'a MyError {}

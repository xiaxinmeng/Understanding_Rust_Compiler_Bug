 rust
#![feature(specialization)]

use std::str::FromStr;

struct Error;

trait Simple<'a> {
    fn do_something(s: &'a str) -> Result<Self, Error>;
}

impl<'a> Simple<'a> for &'a str {
     fn do_something(s: &'a str) -> Result<Self, Error> {
        Ok(s)
    }
}

impl<'a, T: FromStr> Simple<'a> for T {
    fn do_something(s: &'a str) -> Result<Self, Error> {
        T::from_str(s).map_err(|_| Error)
    }
}

fn main() {
    // Do nothing. Just type check.
}

rust
#![feature(test)]

#[derive(Copy, Clone)]
pub enum Error {
    NotFound,
    Busy,
    Timeout,
    Overflow,
}

impl Error {
    #[inline(never)]
    pub fn strerror(self) -> &'static str {
        match self {
            Error::NotFound => "NotFound",
            Error::Busy => "Busy",
            Error::Timeout => "Timeout",
            Error::Overflow => "Overflow",
        }
    }
}

fn main() {
    let f: fn(Error) -> &'static str = std::hint::black_box(Error::strerror);
    std::process::exit(f(Error::Timeout).len() as i32);
}

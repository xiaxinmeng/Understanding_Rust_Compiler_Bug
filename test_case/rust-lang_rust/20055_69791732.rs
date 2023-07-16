 rust
use std::boxed::Box;

#[cfg(not(workaround))]
pub fn main() {
    let _: Box<[i8]> = match true {
        true => Box::new([1i8, 2, 3]),
        false => Box::new([1i8]),
    };
}

#[cfg(workaround)]
pub fn main() {
    let _: Box<[i8]> = match true {
        true => Box::new([1i8, 2, 3]),
        false => { Box::new([1i8]) }
    };
}

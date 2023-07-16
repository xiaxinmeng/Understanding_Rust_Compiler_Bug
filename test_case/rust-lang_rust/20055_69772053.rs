
use std::boxed::Box;

pub fn main() {
    let _: Box<[i8]> = match true { true => Box::new([1i8, 2, 3]), false => Box::new([1i8]) };
}

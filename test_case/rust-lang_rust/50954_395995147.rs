rust
// #![feature(type_ascription)]

use std::borrow::Cow;

fn get_cow(string: &String) -> Cow<str> {
    // Fails on nightly with "type annotations required: 
    // cannot resolve `std::borrow::Cow<'_, str>: std::convert::From<&_>`"
    // https://github.com/rust-lang/rust/issues/50954
    // This works fine in rust 1.27 and earlier
    Cow::from(string.as_ref())
    
    // In rust nightly-1.28 It is easy to fix as in examples below
    // Cow::from(string.as_ref(): &str)
    // Cow::from(string)
    
    // But I found only these are acceptable in both cases
    // Cow::from(string.as_str())
    // Cow::Borrowed(string.as_ref())
    // Cow::Borrowed(string)
}

fn main() {
    let owned = String::from("Hello, world!");
    let _cow = get_cow(&owned);
}

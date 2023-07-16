rust
#![feature(optin_builtin_traits)]

pub auto trait Something {}

// Uncomment this to make the code compile
//
//impl Something for dyn std::error::Error + std::marker::Send + std::marker::Sync + 'static {}

pub fn foo<T: Something>(_: T) {}

#[test]
fn test() {
    use std::io::*;
    foo(Error::new(ErrorKind::Other, ""));
}

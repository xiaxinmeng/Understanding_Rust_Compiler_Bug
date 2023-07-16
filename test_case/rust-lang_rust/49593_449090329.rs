rust
#![feature(never_type)]
pub fn foo(x: !) -> Box<std::error::Error> {
    Box::new(x)
}

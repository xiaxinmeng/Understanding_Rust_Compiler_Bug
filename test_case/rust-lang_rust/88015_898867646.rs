rust
#![feature(if_let_guard)]

pub enum Test {
    A,
    B,
}

pub fn test() -> bool {
    match Test::A {
        Test::A | Test::B if let x = false =>
            x,
        _ => true
    }
}

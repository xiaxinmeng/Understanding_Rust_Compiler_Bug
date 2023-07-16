rust
#![allow(incomplete_features)]
#![feature(inline_const)]

const fn one() -> i32 {
    1
}

fn main() {
    match const { one() } {
        1 => {}
        _ => {}
    }
}

rust
#![feature(inline_const)]

fn main() {
    let vals = const { &[1] };
}

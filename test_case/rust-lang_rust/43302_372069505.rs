rust
#![feature(fn_must_use)]

fn main() {
    let c = || { true };
    c();
}

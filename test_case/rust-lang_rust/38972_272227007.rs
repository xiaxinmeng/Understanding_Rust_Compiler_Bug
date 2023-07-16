rust
#![feature(never_type)]

fn foo(x: Result<u32, !>) {
    let Ok(y) = x;
}

fn main() {
}

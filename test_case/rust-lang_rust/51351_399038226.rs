rust
#![feature(nll)]

fn crash<'a>() {
    let x: &'a str = "?";
}

fn main() {}

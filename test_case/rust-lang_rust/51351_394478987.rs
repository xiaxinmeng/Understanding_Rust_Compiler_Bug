Rust
#![feature(nll)]

fn crash<'a>() {
    let x: &'a () = &();
}

fn main() {}

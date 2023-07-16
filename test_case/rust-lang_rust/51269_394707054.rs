Rust
#![feature(nll)]

fn main() {
    let f: &'static () = &loop {break};
}

 rust
use std::rt::io;

fn foo(r: &io::Writer) {
    r.write([])
}

fn main() {}

 Rust
use std::io;

fn bar(r:Box<Reader>) { }

fn main() {
    let r : Box<Reader> = box io::stdin();
    let _m = bar(r as Box<Reader>);
}

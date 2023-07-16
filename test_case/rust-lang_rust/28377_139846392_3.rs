 Rust
extern crate bar;
extern crate foo; // note different order
extern crate baz;

fn main() {
    let t: Option<Box<baz::BazT>> = None;
}

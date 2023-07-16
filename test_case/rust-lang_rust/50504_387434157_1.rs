rust
// bar.rs
#[macro_use]
extern crate foo;

struct Something;

#[derive(Foo)]
struct Another;

fn main() {}

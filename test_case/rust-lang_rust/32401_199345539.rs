 rust
// main.rs
#[path = "foo.rs"]
mod foo;
fn main() {}

// foo.rs
#[path = "bar.rs"]
mod bar;

// bar.rs
// empty

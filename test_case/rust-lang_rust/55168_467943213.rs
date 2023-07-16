rust
use repro_derive::Example;

#[derive(Example)]
#[example::attr]     // Does not work
struct Demo {
    #[example::attr] // Works
    field: i32,
}

fn main() {}

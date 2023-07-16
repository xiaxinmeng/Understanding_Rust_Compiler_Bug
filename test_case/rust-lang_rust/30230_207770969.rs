 rust
#[derive(Debug)]
struct Foo<'a, T = &'a str> (&'a str, T);

fn main() {}

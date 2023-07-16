rust
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Foo;

#[derive(Debug)]
struct Bar;

type H = HashMap<String, Vec<(Foo, Bar)>>;

fn wat(h: &H) -> H {
    let h: H = h.clone();
    h
}

fn main() {
    let h = H::new();
    println!("{:?}", wat(&h));
}

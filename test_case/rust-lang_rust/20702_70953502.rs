 rust
struct T;
impl T { fn method(&self) {} }

type U = T;

fn foo<U>(x: U) {
    x.method();
}

fn main() {}

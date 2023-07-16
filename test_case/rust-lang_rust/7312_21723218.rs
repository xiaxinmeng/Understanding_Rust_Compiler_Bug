 rust
trait Newable { 
    pub fn new() -> Self;
}

struct Foo;

impl Newable for Foo {
    pub fn new() -> Foo { Foo }
}

impl Foo {
    pub fn new(x: uint) -> Foo { Foo }
}

fn main() {
    printfln!("%?, %?", Newable::new::<Foo>(), Foo::new(42));
}

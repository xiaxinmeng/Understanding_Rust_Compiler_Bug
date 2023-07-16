 rust
trait Foo {
    fn foo(self) { println!("default") }
}

impl Foo for Box<Foo> {
    fn foo(self) { println!("overridden") }
}

impl Foo for int {
    fn foo(self) { println!("int") }
}

fn main() {
    let f = box 1i as Box<Foo>;
    f.foo();
}

rust
// compile-flags: -Copt-level=3 -Cpasses=lint

trait Foo {
    extern fn borrow(&self);
}

struct Bar;
impl Foo for Bar {
    extern fn borrow(&self) {}
}

fn main() {
    let foo: Box<dyn Foo> = Box::new(Bar);
    foo.borrow();
}

 rust
trait Foo {
    fn foo(&self);
}

impl Foo for uint {
    fn foo(&self) {
        println!("foo: {}", *self);
    }
}

pub fn main() {
    let x: Box<Foo+Send> = box 42u;
    let y: &Box<Foo+Send> = &x;
    let z: &Box<Foo> = y;
    z.foo();
}

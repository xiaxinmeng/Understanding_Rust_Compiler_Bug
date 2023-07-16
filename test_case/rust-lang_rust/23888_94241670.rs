 rust
trait Foo { fn foo(&self) { } }
impl Foo for i32 {}

fn main() {
    let zero = 0;
    let foo = (&zero) as &Foo as *const Foo;
    let bar = (&zero) as &Foo as *const Foo;
    foo == bar;
}

 rust
trait Foo { fn foo(&self) { } }
impl Foo for i32 {}

fn main() {
    let foo = (&0) as &Foo as *const Foo;
    let bar = (&0) as &Foo as *const Foo;
    foo == bar;
}

rust
trait Foo: Sized { }

fn foo(_: &Foo) { panic!() }

fn main {
    let x: Box<Foo>;
}

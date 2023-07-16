
use std::rc::Rc;

struct Foo {
    foo: Box<Deref<int> + 'static>,
}

fn main() {
    let foo = Foo{foo: box Rc::new(0) };
    println!("{}", (*foo.foo).deref()); // I work
    println!("{}", **foo.foo); // I don't
}

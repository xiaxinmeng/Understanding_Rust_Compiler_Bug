rust
use std::borrow::BorrowMut;

trait Foo {}

fn repro(mut foo: Box<Foo>) {
    let borrowed: &mut Foo = foo.borrow_mut();
}

fn main() {}

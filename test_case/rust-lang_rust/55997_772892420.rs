
#![feature(type_alias_impl_trait)]

use std::rc::Rc;

type Foo = impl Fn() -> usize;

fn foo() -> Foo {
    let rc = Rc::new(5);
    move || *rc.as_ref()
}

fn assert_send<T: Send>(_: &T) {}

fn main() {
    let f = foo();
    assert_send(&f);
    println!("{}", f());
}

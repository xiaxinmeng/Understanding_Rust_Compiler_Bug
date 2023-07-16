rs
#![feature(thin_box)]
use std::boxed::ThinBox;

pub trait Foo {}

pub fn add_foo(foos: &mut Vec<ThinBox<dyn Foo + '_>>, foo: ThinBox<dyn Foo + 'static>) {
    foos.push(foo);
}

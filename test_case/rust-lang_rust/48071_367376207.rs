rust
#![allow(warnings)]
#![feature(dyn_trait)]
#![feature(nll)]

trait Foo {
    fn foo(&self) { }
}

impl Foo for () {
}

type MakeFooFn = for<'a> fn(&'a u8) -> Box<dyn Foo + 'a>;

fn make_foo(x: &u8) -> Box<dyn Foo + 'static> {
    Box::new(())
}

fn main() {
    let x: MakeFooFn = make_foo as MakeFooFn;
}

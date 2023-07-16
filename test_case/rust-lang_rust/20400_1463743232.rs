
#![feature(associated_const_equality)]

trait Foo { const ENABLED: bool; }

trait Bar {}

impl<T> Bar for T where T: Foo<ENABLED = true> {}
impl<T> Bar for T where T: Foo<ENABLED = false> {}

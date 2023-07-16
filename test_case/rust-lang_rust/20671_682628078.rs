rust
#![feature(trait_alias)]

trait Foo<T: ?Sized> {
   fn foo(&self) -> &T;
}

trait Bar<A> where A: Foo<Self> {}

// This doens't compile
// fn foobar_original<A, B: Bar<A>>(a: &A) -> &B {
//   a.foo()
// }

trait BarAlias<A> = Bar<A> where A: Foo<Self>;
// This does compile
fn foobar_alias<A, B: BarAlias<A>>(a: &A) -> &B {
  a.foo()
}

trait BarAliasExt<A>: BarAlias<A> {}
// This also compiles
fn foobar_super<A, B: BarAliasExt<A>>(a: &A) -> &B {
  a.foo()
}

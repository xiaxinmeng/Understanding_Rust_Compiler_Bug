rust
#![feature(generic_associated_types)]
trait Foo { type Item<'a>; }

// this has an implicit bound `S: 's`.
struct MyFnGood<'s, S: Foo>(&'s dyn Fn(S));

// requires an explicit bound `S: 's`.
struct MyFnBad<'s, S: Foo>(&'s dyn for<'a> Fn(S::Item<'a>));

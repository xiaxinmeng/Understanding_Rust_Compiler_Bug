rust
#![feature(unsized_locals)]

fn drop<T: ?Sized>(_: T) {} // if you remove this line...
trait A {}
const FOO: fn(dyn A) = drop;
// then you will get:  ^^^^ doesn't have a size known at compile-time

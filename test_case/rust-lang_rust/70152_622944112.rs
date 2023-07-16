rust
use std::marker::PhantomData;

trait A {}

struct Foo;

impl A for &'static Foo {}

struct AssertImpl<T: A>(PhantomData<T>);

pub fn f<'a>(_: &'a ()) {
    let _: AssertImpl::<&Foo>; 
}

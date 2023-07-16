rust
#![recursion_limit="10"]

trait Trait {}

struct Foo<X>(X) where for<'a> &'a X: Trait;

impl<X> Foo<X> where for<'a> &'a X: Trait {
    fn new(x: X) -> Foo<X> {
        Foo(x)
    }
}

struct Bar<Y>(Y);

impl<Y> Trait for &Bar<Y> where for<'a> &'a Y: Trait {}

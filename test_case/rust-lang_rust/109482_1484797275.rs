rust
use core::any::Any;

trait Trait {
    type Assoc;
}

impl<X: 'static> Trait for Vec<X> {
    type Assoc = ();
}

struct Foo<T: Trait>(T)
where
    T::Assoc: Clone; // any predicate using `T::Assoc` works here

fn foo<X>(x: X, _: Foo<Vec<X>>) -> Box<dyn Any> {
    Box::new(x) // gets to assume `X: 'static` from the bound on the impl
}

 rust
use std::marker::PhantomData;

trait Id_ {
    type Out;
}

type Id<T> = <T as Id_>::Out;

impl<T> Id_ for T {
    type Out = T;
}

fn coerce<A>(_: PhantomData<A>, x: A) -> A { x }

#[test]
fn test() {
    type Bool = Id<bool>;
    let x = coerce(PhantomData::<Bool>, true);
}

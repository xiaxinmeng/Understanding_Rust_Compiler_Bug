rust
use std::marker::PhantomData;

trait Foo {}

struct Function<F> {
    function: F
}

impl<F, I, O> Foo for Function<F>
    where F: Fn(I) -> O
{}

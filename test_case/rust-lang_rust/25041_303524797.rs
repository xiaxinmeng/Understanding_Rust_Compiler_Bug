rust
use std::marker::PhantomData;

trait Foo {}

struct Function<F, I, O> {
    in_p: PhantomData<I>,
    out_p: PhantomData<O>,
    function: F
}

impl<F, I, O> Foo for Function<F, I, O>
    where F: Fn(I) -> O
{}

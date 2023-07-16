rust
#![recursion_limit="5"] // without it the error is the same, but much bigger

struct Nil;
struct Cons<H, T>(H, T);

pub trait Map<F> {}

impl<F, H, M, T> Map<F> for Cons<H, Cons<M, T>>
where
    Cons<(), T>: Map<Cons<(), ()>>,
{}

impl<F, H, T> Map<F> for Cons<H, T> {}

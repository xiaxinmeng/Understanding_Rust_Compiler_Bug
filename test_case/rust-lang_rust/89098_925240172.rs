rust
// In lib1.rs

pub struct Struct<A, B, C>(A, B, C);

// NOTE: would it make sense to define `struct B`, `struct A`, `struct C`, in that order,
// to make sure the symbol indices are in a random order?

pub trait Trait<Other> {}

impl<A, B, C> Trait<Struct<A, B, C>> for Struct<A, B, C>
where
    A: Trait<A>,
    B: Trait<B>,
    C: Trait<C>,
{
}

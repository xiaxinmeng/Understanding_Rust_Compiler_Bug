rust
#![feature(generic_associated_types)]

pub trait Trait {
    type Gat<'a>;
}

fn fun<T, U, F>()
where
    T: Trait,
    U: Trait,
    F: for<'a> FnOnce(T::Gat<'a>) -> U::Gat<'a>,
{
}

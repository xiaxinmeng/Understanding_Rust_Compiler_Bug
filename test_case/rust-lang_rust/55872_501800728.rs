rust
#![feature(existential_type)]
pub trait Bar
{
    type E: Copy;

    fn foo<T>() -> Self::E;
}

impl <S: Default> Bar for S
{
    existential type E: Copy;

    fn foo<T: Default>() -> Self::E
    {
        (S::default(), T::default())
    }
}

fn main() {}

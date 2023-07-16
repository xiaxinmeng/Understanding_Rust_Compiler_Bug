rust
#![feature(existential_type)]
pub trait Bar
{
    type E: Copy;

    fn foo<T>() -> Self::E;
}

impl <S> Bar for S
{
    existential type E: Copy;

    fn foo<T>() -> Self::E
    {
        || ()
    }
}

fn main() {}

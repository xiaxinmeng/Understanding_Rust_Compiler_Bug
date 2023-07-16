
#![feature(async_await, existential_type)]
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
        async {}
    }
}

fn main() {}
